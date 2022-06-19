# guard4API
Guard4API is a platform for managing APIs. By fronting services with a proxy layer, it provides an abstraction or facade for backend service APIs and provides security, rate limiting, quotas, analytics, throttling and more.

### Http Request format
``` 
Method Request-URI HTTP-Version CRLF
headers CRLF
CRLF
message-body
```

### Http Response format
``` 
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
CRLF
message-body
```
### Echo Server 
```
nc -l localhost 8888
```

### How it works ?
```
1. the guard4API listen incomming request from the Http client
2. Parse the header information 
3. Write the header to the target server ( Http Server )
4. Read body/payload in chunk by chunk 
5. Write every chunk to the server 
6. Read response from the server 
7. Forward to the requested client.

```