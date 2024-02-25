# hifive-demo-server
## Example usage
$ curl -k -X PATCH -H "Content-Type: application/json" -d '{"value": "BLUE"}' http://192.168.132.165:8000/api/command

{"status":"success","data":{"command":{"value":"BLUE","updatedAt":"2024-02-25T21:51:23.550516857Z"}}}
