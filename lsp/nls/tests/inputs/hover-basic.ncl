### /main.ncl
let record = {
  foo | doc "middle" = {
    bar | Number | doc "innermost" = 3
  }
}
in
record.foo.bar
### [[request]]
### type = "Hover"
### textDocument.uri = "file:///main.ncl"
### position = { line = 6, character = 2 }
### [[request]]
### type = "Hover"
### textDocument.uri = "file:///main.ncl"
### position = { line = 6, character = 8 }
### [[request]]
### type = "Hover"
### textDocument.uri = "file:///main.ncl"
### position = { line = 6, character = 12 }
### [[request]]
### type = "Hover"
### textDocument.uri = "file:///main.ncl"
### position = { line = 1, character = 3 }
### [[request]]
### type = "Hover"
### textDocument.uri = "file:///main.ncl"
### position = { line = 2, character = 5 }
