# Internal Error Codes


## Database Error Codes
| Error    | Code |
| -------- | ------- |
| SQLXError         | 1200 |
| RedisError        | 1201 |
| UUIDError         | 1202 |
| TokioError        | 1203 |
| UserNotFound      | 1204 |
| UserAlreadyExists | 1205 |



## Password Error Codes
| Error    | Code |
| -------- | ------- |
| PasswordTooShort                  | 1100 |
| PasswordNoUppercase               | 1101 |
| PasswordNoSymbol                  | 1102 |
| PasswordNoNumber                  | 1103 |
| PasswordNotAscii                  | 1104 |
| PasswordContainsSpecialCharacters | 1105 |
| PasswordContainsWhitespaces       | 1106 |
| PasswordTooLong                   | 1107 |
| PasswordDoesNotMatchHash          | 1108 |
| HashError                         | 1109 |

## Auth Error Codes
| Error    | Code |
| -------- | ------- |
| TokenCreation | 1300 |

## Verification Error Codes
| Error    | Code |
| -------- | ------- |
| InvalidToken | 1400 |
| ExpiredToken | 1401 |
| JWTError     | 1402 |