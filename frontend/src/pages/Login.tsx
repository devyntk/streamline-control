import React from "react";
import { Form, Button, Panel, IconButton, Stack, Divider } from "rsuite";
import { Link } from "react-router-dom";

export default function Login() {
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
        <Form fluid>
          <Form.Group>
            <Form.ControlLabel>Username</Form.ControlLabel>
            <Form.Control name="name" />
          </Form.Group>
          <Form.Group>
            <Form.ControlLabel>
              <span>Password</span>
            </Form.ControlLabel>
            <Form.Control name="name" type="password" />
          </Form.Group>
          <Form.Group>
            <Stack spacing={6} divider={<Divider vertical />}>
              <Button appearance="primary">Sign in</Button>
            </Stack>
          </Form.Group>
        </Form>
      </Panel>
    </Stack>
  );
}
