from turtle import pos
import requests
import unittest

class TestRequests(unittest.TestCase):

    def test_send_message(self):
        url = "http://localhost:8000/message/received"

        message = {
        "message": "This is my test message",
        "user": "",
        "conversation_id": "242865f2-c84e-4e76-a7eb-f230f10bb79b"
        }

        post_request = requests.post(url, json=message)
        self.assertEqual(post_request.status_code, 200)
    
    def test_register_user(self):
        url = "http://localhost:8000/message/register"
        test_user = {
        "user_name": "Python_test_user",
        "password": "Python_test_password",
        "email": "Python_test_emaill@trying.com",
        "id": ""
        }
        post_request = requests.post(url, json=test_user)
        self.assertEqual(post_request.status_code, 200)
    
    def test_remove_user_from_datbase(self):
        url = "http://localhost:8000/message/remove"
        test_user = {
        "user_name": "Python_test_user",
        "password": "Python_test_password",
        "email": "Python_test_emaill@trying.com",
        }
        post_request = requests.post(url, json=test_user)
        print(post_request.text)
        self.assertEqual(post_request.text, "true")

    def test_create_conversation(self):
        url = "http://localhost:8000/conversation/create_conversation"
        test_user = {
        "user_id": "Python_user_1",
        "users_to_invite": ["python_user_2"],
        "public": False
        }
        post_request = requests.post(url, json=test_user)
        print(post_request.status_code)
        assert post_request.status_code == 200, "Failed to create conversation"

if __name__ == "__main__":
    unittest.main()