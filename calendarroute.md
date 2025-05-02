a@a:~/calendly$ curl -X POST http://127.0.0.1:8080/api/calendar/settings -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODE0ZjE2N2QyMjY1YjNiNGExZDZmOTciLCJleHAiOjE3NDY4MTc1MTgsImlhdCI6MTc0NjIxMjcxOCwiZW1haWwiOiJhbmF2aGVvYmFhYnJhaGFtQGdtYWlsLmNvbSJ9.KlPxzoAHFEPAEyW6prybtZhRDuGoUpm7QsyQSP4g7VA" -H "Content-Type: application/json" -d '{"timezone":"UTC+1","working_hours":{"monday":[{"start":"10:00","end":"18:00"}],"tuesday":[{"start":"10:00","end":"18:00"}],"wednesday":[{"start":"10:00","end":"18:00"}],"thursday":[{"start":"10:00","end":"18:00"}],"friday":[{"start":"10:00","end":"18:00"}]},"buffer_time":{"before":10,"after":10},"default_meeting_duration":45,"calendar_name":"My Professional Calendar","date_format":"DD/MM/YYYY","time_format":"12h"}'
{"id":"6815186a02743543884ca3e5","user_id":"6814f167d2265b3b4a1d6f97","timezone":"UTC+1","working_hours":{"wednesday":[{"start":"10:00","end":"18:00"}],"tuesday":[{"start":"10:00","end":"18:00"}],"friday":[{"start":"10:00","end":"18:00"}],"thursday":[{"start":"10:00","end":"18:00"}],"monday":[{"start":"10:00","end":"18:00"}]},"buffer_time":{"before":10,"after":10},"default_meeting_duration":45,"calendar_name":"My Professional Calendar","date_format":"DD/MM/YYYY","time_format":"12h","created_at":"2025-05-02 19:09:30.209 +00:00:00","updated_at":"2025-05-02 19:09:30.209 +00:00:00"}a@a:~/calendly$ 














a@a:~/calendly$ curl -X PUT http://127.0.0.1:8080/api/calendar/settings -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODE0ZjE2N2QyMjY1YjNiNGExZDZmOTciLCJleHAiOjE3NDYyMTE3MDgsImlhdCI6MTc0NjIxMDgwOCwiZW1haWwiOiJhbmF2aGVvYmFhYnJhaGFtQGdtYWlsLmNvbSJ9.aiy2AngdrDBSuA-qOvIafmKa3770pYXYo16352Q4Tmg" -H "Content-Type: application/json" -d '{"timezone":"UTC+1","working_hours":{"monday":[{"start":"08:00","end":"16:00"}],"tuesday":[{"start":"08:00","end":"16:00"}],"wednesday":[{"start":"08:00","end":"16:00"}],"thursday":[{"start":"08:00","end":"16:00"}],"friday":[{"start":"08:00","end":"16:00"}]},"buffer_time":{"before":15,"after":15},"default_meeting_duration":30,"calendar_name":"Updated Calendar","date_format":"DD/MM/YYYY","time_format":"24h"}'
{"id":"681510a1c7104c113b1cb04d","user_id":"6814f167d2265b3b4a1d6f97","timezone":"UTC+1","working_hours":{"wednesday":[{"start":"09:00","end":"17:00"}],"thursday":[{"start":"09:00","end":"17:00"}],"tuesday":[{"start":"09:00","end":"17:00"}],"friday":[{"start":"09:00","end":"17:00"}],"monday":[{"start":"09:00","end":"17:00"}]},"buffer_time":{"before":15,"after":15},"default_meeting_duration":30,"calendar_name":"My Calendar","date_format":"DD/MM/YYYY","time_format":"24h","created_at":"2025-05-02 18:36:17.731 +00:00:00","updated_at":"2025-05-02 18:36:17.731 +00:00:00"}a@a:~/calendly$ 







a@a:~/calendly$ curl -X DELETE http://127.0.0.1:8080/api/calendar/settings -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2ODE0ZjE2N2QyMjY1YjNiNGExZDZmOTciLCJleHAiOjE3NDY4MTc1MTgsImlhdCI6MTc0NjIxMjcxOCwiZW1haWwiOiJhbmF2aGVvYmFhYnJhaGFtQGdtYWlsLmNvbSJ9.KlPxzoAHFEPAEyW6prybtZhRDuGoUpm7QsyQSP4g7VA"
{"message":"Calendar settings deleted successfully"}a@a:~/calendly$ 
