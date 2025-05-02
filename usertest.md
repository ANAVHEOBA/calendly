a@a:~/calendly$ curl -X POST http://127.0.0.1:8080/api/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "anavheobaabraham@gmail.com",
    "password": "your_password_here",
    "name": "Wisdom Abraham"
  }'
{"message":"Registration successful! Please check your email for a verification code.","user":{"id":"6814f167d2265b3b4a1d6f97","email":"anavheobaabraham@gmail.com","name":"Wisdom Abraham","is_verified":false}}a@a:~/calendly$ 






a@a:~/calendly$ curl -X POST http://127.0.0.1:8080/api/users/verify-email \
  -H "Content-Type: application/json" \
  -d '{
    "token": "006583"
  }'
{"message":"Email verified successfully"}a@a:~/calendly$ 





a@a:~/calendly$ curl -X POST http://127.0.0.1:8080/api/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "anavheobaabraham@gmail.com",
    "password": "your_password_here"
  }'
{"access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODE0ZjE2N2QyMjY1YjNiNGExZDZmOTciLCJleHAiOjE3NDY4MTc1MTgsImlhdCI6MTc0NjIxMjcxOCwiZW1haWwiOiJhbmF2aGVvYmFhYnJhaGFtQGdtYWlsLmNvbSJ9.KlPxzoAHFEPAEyW6prybtZhRDuGoUpm7QsyQSP4g7VA","refresh_token":"5RVYnFXZ07X1beNReyjjWotalT3zbhDB","user":{"id":"6814f167d2265b3b4a1d6f97","email":"anavheobaabraham@gmail.com","name":"Wisdom Abraham","is_verified":true}}a@a:~/calendly$ clear








a@a:~/calendly$ curl -X POST http://127.0.0.1:8080/api/users/refresh-token \
  -H "Content-Type: application/json" \
  -d '{"refresh_token": "aYf2pPW9DNtRBaGa3jcyv0oxek8HV5XW"}'
{"access_token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODE0ZjE2N2QyMjY1YjNiNGExZDZmOTciLCJleHAiOjE3NDYyMDUzODQsImlhdCI6MTc0NjIwNDQ4NCwiZW1haWwiOiJhbmF2aGVvYmFhYnJhaGFtQGdtYWlsLmNvbSJ9.SHBh6-XkFE6VLf3ffAyv66jiY_AV0dSAkT_G19p5xZo","refresh_token":"IKGSRQkMJt0y8uRbH0ybpoYEf0WKJddV"}a@a:~/calendly$ 
