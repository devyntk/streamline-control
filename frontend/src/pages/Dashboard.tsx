import {
  Container,
  Sidebar,
  Sidenav,
  Content,
  Nav,
  Navbar,
  Stack,
} from "rsuite";
import ArrowLeftLineIcon from "@rsuite/icons/ArrowLeftLine";
import ArrowRightLineIcon from "@rsuite/icons/ArrowRightLine";
import React, { useEffect, useState } from "react";
import { Outlet, useNavigate } from "react-router";
import { Link, NavLinkProps, NavLink } from "react-router-dom";
import { useAuthStore } from "../stores/auth";

export default function Dashboard() {
  const [expand, setExpand] = useState(true);

  const auth = useAuthStore();
  const navigate = useNavigate();
  useEffect(() => {
    if (auth.sessionKey === null) {
      navigate("/login");
    }
  }, [auth.sessionKey]);

  return (
    <Container className="h-screen">
      <Sidebar
        style={{ display: "flex", flexDirection: "column" }}
        width={expand ? 260 : 56}
        collapsible
      >
        <Sidenav.Header>
          <Stack>
            <Link to="/">
              <span className={"text-xl "}>Streamline Control</span>
            </Link>
          </Stack>
        </Sidenav.Header>
        <Sidenav
          expanded={expand}
          appearance="subtle"
          defaultOpenKeys={["2", "3"]}
        >
          <Sidenav.Body>
            <Nav>
              <Nav.Item to="/" as={NavLink}>
                Home
              </Nav.Item>
              <Nav.Item to="/status" as={NavLink}>
                Status
              </Nav.Item>
              <Nav.Item
                onClick={() => {
                  auth.logout();
                }}
              >
                Logout
              </Nav.Item>
            </Nav>
          </Sidenav.Body>
        </Sidenav>
        <Navbar appearance="subtle" className="nav-toggle">
          <Nav pullRight>
            <Nav.Item
              onClick={() => {
                setExpand(!expand);
              }}
              style={{ textAlign: "center" }}
              icon={expand ? <ArrowLeftLineIcon /> : <ArrowRightLineIcon />}
            />
          </Nav>
        </Navbar>
      </Sidebar>

      <Container>
        <Content style={{ backgroundColor: "#f5f8fa" }}>
          <Outlet />
        </Content>
      </Container>
    </Container>
  );
}
