use anyhow::Result;
use clap::Parser;
use indexmap::IndexMap;
use openapiv3::{Components, Parameter, ReferenceOr, RequestBody, Response, Schema};
use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

/// A command line tool that processes OpenAPI specifications
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the OpenAPI specification file
    #[arg(short, long, value_name = "FILE")]
    spec: PathBuf,

    /// Print suggested cursor position.
    ///
    /// If this option is set, the output will be in JSON format with metadata
    #[arg(short, long)]
    json: bool,

    /// Read input from stdin if provided
    #[arg(hide = true)]
    stdin_input: Option<String>,
    // TODO probably add an option so you can specify a prefix on the paths
}

#[derive(Debug, serde::Serialize)]
struct OutputMetadata {
    cursor_position: usize,
    stdout: String,
}

fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    let json_out = args.json;

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("Failed to read from stdin");

    let mut a = buffer.split('|');
    let curl_command_position = a.position(|part| part.trim().starts_with("curl"));
    if curl_command_position.is_none() {
        // If no curl command is found, just print the input and exit
        io::stdout().write_all(buffer.as_bytes())?;
        return Ok(());
    }
    let curl_command_position = curl_command_position.unwrap();
    let curl_command = buffer.split('|').nth(curl_command_position).unwrap().trim();

    let spec_path = args.spec;
    if !spec_path.exists() {
        io::stdout().write_all(buffer.as_bytes())?;
        std::process::exit(1);
    }
    let spec_content =
        std::fs::read_to_string(&spec_path).expect("Failed to read OpenAPI specification file");
    if spec_content.is_empty() {
        io::stdout().write_all(buffer.as_bytes())?;
        std::process::exit(1);
    }
    // For JSON spec
    let spec: Result<openapiv3::OpenAPI, anyhow::Error> = if spec_path.ends_with(".json") {
        serde_json::from_str(&spec_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON OpenAPI spec: {}", e))
    } else {
        // For YAML spec
        serde_yaml::from_str(&spec_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse YAML OpenAPI spec: {}", e))
    };
    if let Err(e) = spec {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("Error parsing OpenAPI spec: {}", e);
        std::process::exit(1);
    }
    let spec = spec.unwrap();

    let parsed_request = curl_parser::ParsedRequest::from_str(curl_command);
    if let Err(e) = parsed_request {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("Error parsing curl command: {}", e);
        std::process::exit(1);
    }
    let mut parsed_request = parsed_request.unwrap();
    parsed_request.headers.remove(http::header::ACCEPT);
    let path = parsed_request.url.path();
    let match_path = spec.paths.paths.get(path);
    if match_path.is_none() {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("No matching path found in OpenAPI spec for path: {}", path);
        std::process::exit(1);
    }
    let match_path = match_path.unwrap().as_item();
    if match_path.is_none() {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("No matching path found in OpenAPI spec for path: {}", path);
        std::process::exit(1);
    }
    let match_path = match_path.unwrap();
    let method = parsed_request.method.as_str();
    let operation = match method {
        "GET" => &match_path.get,
        "POST" => &match_path.post,
        "PUT" => &match_path.put,
        "DELETE" => &match_path.delete,
        "PATCH" => &match_path.patch,
        "HEAD" => &match_path.head,
        "OPTIONS" => &match_path.options,
        _ => &None,
    };
    if operation.is_none() {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!(
        //    "No matching operation found in OpenAPI spec for method: {}",
        //    method
        //);
        std::process::exit(1);
    }
    let operation = operation.as_ref().unwrap();
    let parameters = parameter_map(&operation.parameters, &spec.components);
    if let Err(e) = parameters {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("Error processing parameters: {}", e);
        std::process::exit(1);
    }
    let parameters = parameters.unwrap();
    let first_empty_spec_parameter = get_first_empty_spec_parameter(&parameters, &parsed_request);
    let populated_header_names = parsed_request
        .headers
        .iter()
        .filter_map(|(name, value)| {
            if value.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect::<Vec<_>>();
    let populated_query_names = parsed_request
        .data_url_encoded
        .iter()
        .filter_map(|(name, value)| {
            if value.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect::<Vec<_>>();

    match first_empty_spec_parameter {
        Some(empty_parameter) => {
            let replacement_paremeter = match empty_parameter {
                EmptySpecParameter::Header(name) => {
                    parsed_request.headers.remove(&name);
                    let parameter_position =
                        parameters.iter().position(|(n, _)| *n == &name).unwrap();
                    let mut next_parameter_iter = parameters.iter().cycle();
                    let mut next_parameter =
                        next_parameter_iter.nth(parameter_position + 1).unwrap();
                    let max_iterations = parameters.len();
                    let mut iterations = 0;
                    loop {
                        let next_name = next_parameter.1.parameter_data_ref().name.to_string();
                        if !populated_header_names.contains(&next_name)
                            || iterations >= max_iterations
                        {
                            break;
                        }
                        next_parameter = next_parameter_iter.next().unwrap();
                        iterations += 1;
                    }

                    next_parameter.1
                }
                EmptySpecParameter::Query(name) => {
                    parsed_request.data_url_encoded.remove(&name);
                    let parameter_position =
                        parameters.iter().position(|(n, _)| *n == &name).unwrap();
                    let mut next_parameter_iter = parameters.iter().cycle();
                    let mut next_parameter =
                        next_parameter_iter.nth(parameter_position + 1).unwrap();
                    let max_iterations = parameters.len();
                    let mut iterations = 0;
                    loop {
                        let next_name = next_parameter.1.parameter_data_ref().name.to_string();
                        if !populated_query_names.contains(&next_name)
                            || iterations >= max_iterations
                        {
                            break;
                        }
                        next_parameter = next_parameter_iter.next().unwrap();
                        iterations += 1;
                    }

                    next_parameter.1
                }
            };
            match replacement_paremeter {
                Parameter::Header { parameter_data, .. } => {
                    let name = &parameter_data.name;
                    let value = parameter_data
                        .example
                        .as_ref()
                        .unwrap_or(&serde_json::Value::String("".to_string()))
                        .to_string();
                    let header_value = http::header::HeaderValue::from_str(&value)
                        .unwrap_or_else(|_| http::header::HeaderValue::from_static("invalid"));
                    parsed_request.headers.insert(
                        http::header::HeaderName::from_str(name).unwrap(),
                        header_value,
                    );
                }
                Parameter::Query { parameter_data, .. } => {
                    let name = &parameter_data.name;
                    let value = parameter_data.example.as_ref();
                    let value = match value {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    };
                    parsed_request
                        .data_url_encoded
                        .insert(name.to_string(), value);
                }
                _ => {}
            }
        }
        None => {
            for (_, parameter) in parameters.iter() {
                if let Parameter::Header { parameter_data, .. } = parameter {
                    let name = &parameter_data.name;
                    if parsed_request.headers.contains_key(name) {
                        // If the header is already set, skip it
                        continue;
                    }
                    let value = parameter_data
                        .example
                        .as_ref()
                        .unwrap_or(&serde_json::Value::String("".to_string()))
                        .to_string();
                    let header_value = http::header::HeaderValue::from_str(&value)
                        .unwrap_or_else(|_| http::header::HeaderValue::from_static("invalid"));
                    parsed_request.headers.insert(
                        http::header::HeaderName::from_str(name).unwrap(),
                        header_value,
                    );
                    break;
                } else if let Parameter::Query { parameter_data, .. } = parameter {
                    let name = &parameter_data.name;
                    if parsed_request.data_url_encoded.contains_key(name) {
                        // If the query parameter is already set, skip it
                        continue;
                    }
                    let value = parameter_data.example.as_ref();
                    let value = match value {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    };
                    parsed_request
                        .data_url_encoded
                        .insert(name.to_string(), value);
                    break;
                }
            }
        }
    };

    // Bail early if the request body is already set
    if parsed_request.body().is_some() {
        print_result_and_exit(&parsed_request, json_out, &buffer, curl_command_position);
    }

    let body = operation.request_body.as_ref();
    if body.is_none() {
        // If no request body is defined, just print the request and exit
        print_result_and_exit(&parsed_request, json_out, &buffer, curl_command_position);
    }
    let body = body.as_ref().unwrap();
    let body = body.item(&spec.components);
    if body.is_err() {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("Error retrieving request body: {}", body.unwrap_err());
        std::process::exit(1);
    }
    let body = body.unwrap();
    if body.content.get("application/json").is_none() {
        // If no JSON content is defined, just print the request and exit
        print_result_and_exit(&parsed_request, json_out, &buffer, curl_command_position);
    }
    let media_type = body.content.get("application/json").unwrap();
    parsed_request.headers.insert(
        http::header::CONTENT_TYPE,
        http::header::HeaderValue::from_static("application/json"),
    );
    parsed_request.headers.insert(
        http::header::ACCEPT,
        http::header::HeaderValue::from_static("application/json"),
    );
    let example = media_type.example.as_ref();
    if example.is_some() {
        let example = example.as_ref().unwrap();
        let example_str = serde_json::to_string(example).unwrap_or_else(|_| "{}".to_string());
        parsed_request.body = vec![example_str];
        print_result_and_exit(&parsed_request, json_out, &buffer, curl_command_position);
        return Ok(());
    }

    let schema = media_type.schema.as_ref();
    if schema.is_none() {
        // If no schema is defined, just print the request and exit
        print_result_and_exit(&parsed_request, json_out, &buffer, curl_command_position);
    }
    let schema = schema.as_ref().unwrap();
    let schema = schema.item(&spec.components);
    if schema.is_err() {
        io::stdout().write_all(buffer.as_bytes())?;
        //eprintln!("Error retrieving schema: {}", schema.unwrap_err());
        std::process::exit(1);
    }
    let schema = schema.unwrap();
    if let openapiv3::SchemaKind::Type(t) = &schema.schema_kind {
        if schema.schema_data.example.is_some() {
            // If the schema has an example, use it as the request body
            let example = schema.schema_data.example.as_ref().unwrap();
            let example_str = serde_json::to_string(example).unwrap_or_else(|_| "{}".to_string());
            parsed_request.body = vec![example_str]
        }
    };
    print_result_and_exit(&parsed_request, json_out, &buffer, curl_command_position);

    Ok(())
}

#[derive(Debug)]
enum EmptySpecParameter {
    Header(String),
    Query(String),
}
fn get_first_empty_spec_parameter(
    parameters: &BTreeMap<&String, &Parameter>,
    parsed_request: &curl_parser::ParsedRequest,
) -> Option<EmptySpecParameter> {
    for (name, param) in parameters.iter() {
        if let Parameter::Header { parameter_data, .. } = param {
            if let Some(value) = parsed_request.headers.get(*name) {
                if !value.is_empty() {
                    // If the header is already set, skip it
                    continue;
                }
                return Some(EmptySpecParameter::Header(name.to_string()));
            }
        } else if let Parameter::Query { parameter_data, .. } = param {
            if let Some(value) = parsed_request.data_url_encoded.get(*name) {
                if !value.is_empty() {
                    // If the query parameter is already set, skip it
                    continue;
                }
                return Some(EmptySpecParameter::Query(name.to_string()));
            }
        }
    }
    None
}

fn print_result_and_exit(
    request: &curl_parser::ParsedRequest,
    json_out: bool,
    original_buffer: &str,
    command_position: usize,
) {
    let no_body_with_query_parameters =
        request.body().is_none() && !request.data_url_encoded.is_empty();
    let format_dash_dash_get = if no_body_with_query_parameters {
        "-G "
    } else {
        ""
    };
    let mut request_out = format!(
        "curl -X {} {}{}",
        request.method, format_dash_dash_get, request.url
    );
    for (h, v) in request.headers.iter() {
        let header_value = match v.is_empty() || v.to_str().unwrap_or("") == "\"\"" {
            true => "",
            false => v.to_str().unwrap_or(""),
        };
        dbg!(header_value);
        let header = format!("-H \"{}: {}\"", h, header_value);
        request_out.push_str(&format!(" {}", header));
    }
    if let Some(body) = request.body() {
        let body_str = body.to_string();
        let value = serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(&body_str).unwrap_or_default(),
        )
        .unwrap_or_else(|_| "{}".to_string());
        request_out.push_str(&format!(" -d '{}'", value));
    } else if !request.data_url_encoded.is_empty() {
        let data: Vec<String> = request
            .data_url_encoded
            .iter()
            .map(|(k, v)| format!("--data-urlencode '{}={}'", k, v))
            .collect();
        request_out.push_str(&format!(" {}", data.join(" ")));
    }
    let mut with_cursor_position = request_out.len() - 1;
    let mut commands_slice = original_buffer
        .split('|')
        .map(|c| c.trim())
        .collect::<Vec<_>>();
    commands_slice[command_position] = &request_out;
    let padding_string = " | ";
    let request_out = commands_slice.join(padding_string);
    for cmd in commands_slice[..command_position].iter() {
        with_cursor_position += cmd.len();
    }
    if json_out {
        let metadata = OutputMetadata {
            cursor_position: with_cursor_position,
            stdout: request_out,
        };
        let json_output =
            serde_json::to_string(&metadata).expect("Failed to serialize output metadata to JSON");
        std::io::stdout()
            .write_all(json_output.as_bytes())
            .expect("Failed to write JSON output to stdout");
    } else {
        std::io::stdout()
            .write_all(request_out.as_bytes())
            .expect("Failed to write to stdout");
    }

    std::process::exit(0);
}

pub(crate) trait ReferenceOrExt<T: ComponentLookup> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T>;
}
pub(crate) trait ComponentLookup: Sized {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>>;
}
impl<T: ComponentLookup> ReferenceOrExt<T> for openapiv3::ReferenceOr<T> {
    fn item<'a>(&'a self, components: &'a Option<Components>) -> Result<&'a T> {
        match self {
            ReferenceOr::Item(item) => Ok(item),
            ReferenceOr::Reference { reference } => {
                let idx = reference.rfind('/').unwrap();
                let key = &reference[idx + 1..];
                let parameters = T::get_components(components.as_ref().unwrap());
                parameters
                    .get(key)
                    .unwrap_or_else(|| panic!("key {} is missing", key))
                    .item(components)
            }
        }
    }
}

pub(crate) fn items<'a, T>(
    refs: &'a [ReferenceOr<T>],
    components: &'a Option<Components>,
) -> impl Iterator<Item = Result<&'a T>>
where
    T: ComponentLookup,
{
    refs.iter().map(|r| r.item(components))
}

pub(crate) fn parameter_map<'a>(
    refs: &'a [ReferenceOr<Parameter>],
    components: &'a Option<Components>,
) -> Result<BTreeMap<&'a String, &'a Parameter>> {
    items(refs, components)
        .map(|res| res.map(|param| (&param.parameter_data_ref().name, param)))
        .collect()
}

impl ComponentLookup for Parameter {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.parameters
    }
}

impl ComponentLookup for RequestBody {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.request_bodies
    }
}

impl ComponentLookup for Response {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.responses
    }
}

impl ComponentLookup for Schema {
    fn get_components(components: &Components) -> &IndexMap<String, ReferenceOr<Self>> {
        &components.schemas
    }
}
