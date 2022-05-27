from turtle import pos
import requests
import unittest

class TestRequests(unittest.TestCase):

    def setUp(self) -> None:
        self.test_user = {
            "user_name": "Python_test_user",
            "password": "Python_test_password",
            "email": "Python_test_emaill@trying.com",
            "id": ""
        }
        self.url = "http://localhost:8000/"

    def tearDown(self) -> None:
        self.test_user = {
            "user_name": "Python_test_user",
            "password": "Python_test_password",
            "email": "Python_test_emaill@trying.com",
            "id": ""
        }
        self.url = "http://localhost:8000/"

    def test_send_message(self):
        self.url += "message/received"

        message = {
        "message": "This is my test message",
        "user": "",
        "conversation_id": "242865f2-c84e-4e76-a7eb-f230f10bb79b"
        }

        post_request = requests.post(self.url, json=message)
        self.assertEqual(post_request.status_code, 200)
    
    def test_register_user(self):
        self.url += "message/register"
        post_request = requests.post(self.url, json=self.test_user)
        self.assertEqual(post_request.status_code, 200)
    
    def test_remove_user_from_datbase(self):
        self.url += "message/remove"
        post_request = requests.post(self.url, json=self.test_user)
        print(post_request.text)
        self.assertEqual(post_request.text, "true")

    def test_create_conversation(self):
        self.url += "conversation/create_conversation"
        test_user = {
        "user_id": "Python_user_1",
        "users_to_invite": ["python_user_2"],
        "public": False
        }
        post_request = requests.post(self.url, json=test_user)
        print(post_request.status_code)
        assert post_request.status_code == 200, "Failed to create conversation"

if __name__ == "__main__":
    unittest.main()