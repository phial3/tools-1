# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > interface > index-signature`

```javascript
Program {
  comments: Array []
  corrupt: false
  diagnostics: Array []
  directives: Array []
  filename: 'input.ts'
  hasHoistedVars: false
  interpreter: undefined
  mtime: undefined
  sourceType: 'module'
  syntax: Array ['ts']
  loc: Object {
    filename: 'input.ts'
    end: Object {
      column: 0
      index: 41
      line: 4
    }
    start: Object {
      column: 0
      index: 0
      line: 1
    }
  }
  body: Array [
    TSInterfaceDeclaration {
      id: BindingIdentifier {
        name: 'I'
        loc: Object {
          filename: 'input.ts'
          end: Object {
            column: 11
            index: 11
            line: 1
          }
          start: Object {
            column: 10
            index: 10
            line: 1
          }
        }
      }
      extends: undefined
      typeParameters: undefined
      loc: Object {
        filename: 'input.ts'
        end: Object {
          column: 1
          index: 40
          line: 3
        }
        start: Object {
          column: 0
          index: 0
          line: 1
        }
      }
      body: TSInterfaceBody {
        loc: Object {
          filename: 'input.ts'
          end: Object {
            column: 1
            index: 40
            line: 3
          }
          start: Object {
            column: 12
            index: 12
            line: 1
          }
        }
        body: Array [
          TSIndexSignature {
            key: BindingIdentifier {
              name: 's'
              loc: Object {
                filename: 'input.ts'
                end: Object {
                  column: 14
                  index: 28
                  line: 2
                }
                start: Object {
                  column: 5
                  index: 19
                  line: 2
                }
              }
              meta: PatternMeta {
                loc: Object {
                  filename: 'input.ts'
                  end: Object {
                    column: 14
                    index: 28
                    line: 2
                  }
                  start: Object {
                    column: 5
                    index: 19
                    line: 2
                  }
                }
                typeAnnotation: StringKeywordTypeAnnotation {
                  loc: Object {
                    filename: 'input.ts'
                    end: Object {
                      column: 14
                      index: 28
                      line: 2
                    }
                    start: Object {
                      column: 8
                      index: 22
                      line: 2
                    }
                  }
                }
              }
            }
            readonly: false
            loc: Object {
              filename: 'input.ts'
              end: Object {
                column: 24
                index: 38
                line: 2
              }
              start: Object {
                column: 4
                index: 18
                line: 2
              }
            }
            typeAnnotation: NumberKeywordTypeAnnotation {
              loc: Object {
                filename: 'input.ts'
                end: Object {
                  column: 23
                  index: 37
                  line: 2
                }
                start: Object {
                  column: 17
                  index: 31
                  line: 2
                }
              }
            }
          }
        ]
      }
    }
  ]
}
```