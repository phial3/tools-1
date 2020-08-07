# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > expression-primary-object > migrated_0002`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "esprima/expression-primary-object/migrated_0002/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "esprima/expression-primary-object/migrated_0002/input.js"
		end: Object {
			column: 18
			line: 1
		}
		start: Object {
			column: 0
			line: 1
		}
	}
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "esprima/expression-primary-object/migrated_0002/input.js"
				end: Object {
					column: 18
					line: 1
				}
				start: Object {
					column: 0
					line: 1
				}
			}
			expression: JSAssignmentExpression {
				operator: "="
				loc: Object {
					filename: "esprima/expression-primary-object/migrated_0002/input.js"
					end: Object {
						column: 18
						line: 1
					}
					start: Object {
						column: 0
						line: 1
					}
				}
				left: JSAssignmentIdentifier {
					name: "x"
					loc: Object {
						filename: "esprima/expression-primary-object/migrated_0002/input.js"
						identifierName: "x"
						end: Object {
							column: 1
							line: 1
						}
						start: Object {
							column: 0
							line: 1
						}
					}
				}
				right: JSObjectExpression {
					loc: Object {
						filename: "esprima/expression-primary-object/migrated_0002/input.js"
						end: Object {
							column: 18
							line: 1
						}
						start: Object {
							column: 4
							line: 1
						}
					}
					properties: Array [
						JSObjectProperty {
							key: JSStaticPropertyKey {
								value: JSIdentifier {
									name: "answer"
									loc: Object {
										filename: "esprima/expression-primary-object/migrated_0002/input.js"
										identifierName: "answer"
										end: Object {
											column: 12
											line: 1
										}
										start: Object {
											column: 6
											line: 1
										}
									}
								}
								loc: Object {
									filename: "esprima/expression-primary-object/migrated_0002/input.js"
									end: Object {
										column: 12
										line: 1
									}
									start: Object {
										column: 6
										line: 1
									}
								}
							}
							value: JSNumericLiteral {
								value: 42
								format: undefined
								loc: Object {
									filename: "esprima/expression-primary-object/migrated_0002/input.js"
									end: Object {
										column: 16
										line: 1
									}
									start: Object {
										column: 14
										line: 1
									}
								}
							}
							loc: Object {
								filename: "esprima/expression-primary-object/migrated_0002/input.js"
								end: Object {
									column: 16
									line: 1
								}
								start: Object {
									column: 6
									line: 1
								}
							}
						}
					]
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```