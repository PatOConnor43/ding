# Ding 🛎️

Ding is a project to help write curl commands based on an OpenAPI specification.

It does this by reading an OpenAPI specification, comparing that to a `curl` command (passed in from stdin), then re-writing the `curl` command to include Query, Header, and Body parameters.

## Examples

### Spec
```yaml
openapi: "3.0.0"
info:
  version: 1.0.0
  title: Swagger Petstore
  license:
    name: MIT
servers:
  - url: http://petstore.swagger.io/v1
paths:
  /pets:
    get:
      summary: List all pets
      operationId: listPets
      tags:
        - pets
      parameters:
        - name: limit
          in: query
          description: How many items to return at one time (max 100)
          required: false
          schema:
            type: integer
            format: int32
            example: 100
      responses:
        '200':
          description: A paged array of pets
          headers:
            x-next:
              description: A link to the next page of responses
              schema:
                type: string
          content:
            application/json:    
              schema:
                $ref: "#/components/schemas/Pets"
        default:
          description: unexpected error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Error"
    post:
      summary: Create a pet
      operationId: createPets
      tags:
        - pets
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Pet'
            example:
              name: "Rover"
      responses:
        '201':
          description: Null response
        default:
          description: unexpected error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Error"
components:
  schemas:
    Pet:
      type: object
      required:
        - id
        - name
      properties:
        id:
          type: integer
          format: int64
        name:
          type: string
        tag:
          type: string
    Pets:
      type: array
      items:
        $ref: "#/components/schemas/Pet"
    Error:
      type: object
      required:
        - code
        - message
      properties:
        code:
          type: integer
          format: int32
        message:
          type: string

```

#### Automatically adding query parameters

```bash
echo -n "curl -X GET http://localhost:8080/pets" | ding --spec <path/to/openapi.yaml>
```
output:
```bash
curl -X GET -G 'http://localhost:8080/pets' --data-urlencode 'limit=100'
```

#### Automatically adding request body
```bash
echo -n "curl -X POST http://localhost:8080/pets" | ding --spec <path/to/openapi.yaml>
```
output:
```bash
curl -X POST 'http://localhost:8080/pets' -H 'Content-Type: application/json' -d '{"name":"Rover"}'
```

## Shell Integration

I wrote this so I could actually use it _while_ I'm writing the `curl` command. `zsh` has a feature that allows you to create keybindings that run commands and edit the current buffer. You can add this snippet to your `~/.zshrc` file to do something similar:

```zsh
_ding_widget() {
    DING=`ding --spec <path/to/openapi.yaml> --json <<< "$BUFFER"`
    BUFFER=`jq -r '.stdout' <<< "$DING"`
    CURSOR=`jq -r '.cursor_position' <<< "$DING"`
}

zle -N _ding_widget
bindkey '^X^X' _ding_widget
```
This will allow you to press `Ctrl-X Ctrl-X` to run `ding` on the current command in your shell, and it will replace the command with the output of `ding`.

## Contributing

This project uses cargo insta for snapshot testing. You can run the tests normally with `cargo test`. If you are making changes and want to compare the snapshots you can run `cargo insta test --review`.

### Cargo dist
This project uses `cargo dist` to build a distributable binary. Specifically this project uses the fork maintained by [astral-sh](https://github.com/astral-sh/cargo-dist). To install it, go to the releases page and install the binary.

Updating `dist` can be done with `dist selfupdate`.

### Releasing
In order to tag a new release, follow these steps:
- Checkout master
- Add a changelog for the version to `RELEASES.md`
- Commit the changes
- Run `cargo release patch --no-publish` to dry run the release (minor or major can be used as well)
- Run `cargo release patch --no-publish --execute` to actually tag the release
