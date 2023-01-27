import React, { useState } from "react";
import { Form, Button, Panel, Stack, Divider } from "rsuite";
import axios from "axios";
import { useAuthStore } from "../stores/auth";
import { useNavigate } from "react-router";

export default function Login() {
  const [form, setForm] = useState<Record<string, any>>({
    username: "",
    password: "",
  });

  const auth = useAuthStore();
  const navigate = useNavigate();

  return (
    <Stack
      justifyContent="center"
      alignItems="center"
      direction="column"
      style={{
        height: "100vh",
      }}
    >
      <Panel
        bordered
        style={{ background: "#fff", width: 400 }}
        header={<h3>Sign In</h3>}
      >
        <Form
          fluid
          formValue={form}
          onChange={(formValue) => setForm(formValue)}
        >
          <Form.Group controlId="username">
            <Form.ControlLabel>Username</Form.ControlLabel>
            <Form.Control name="username" />
          </Form.Group>
          <Form.Group controlId="password">
            <Form.ControlLabel>Password</Form.ControlLabel>
            <Form.Control name="password" type="password" />
          </Form.Group>
          <Form.Group>
            <Stack spacing={6} divider={<Divider vertical />}>
              <Button
                appearance="primary"
                onClick={() => {
                  axios
                    .post("/api/auth/login", {
                      username: form.username,
                      password: form.password,
                    })
                    .then((response) => {
                      auth.login(response.data.token);
                      navigate("/");
                    });
                }}
              >
                Sign in
              </Button>
            </Stack>
          </Form.Group>
        </Form>
      </Panel>
    </Stack>
  );
}
