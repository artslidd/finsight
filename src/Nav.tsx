import logo from "./assets/logo_v1.png";

import "./App.scss";
import { useNavigate } from "react-router-dom";

function Nav() {
  const navigate = useNavigate();
  return (
    <>
      <nav className="navbar navbar-expand bd-navbar bg-primary-subtle">
        <div className="container-lg d-flex flex-row justify-content-between">
          <a className="navbar-brand" href="#">
            <img
              src={logo}
              alt="Logo"
              width="40"
              height="30"
              className="d-inline-block align-text-top"
            />
            <a>finsight</a>
          </a>
          <div className="d-flex flex-row">
            <button
              onClick={() => navigate("/add-asset")}
              className="btn btn-outline-primary m-2"
            >
              <i className="bi bi-plus-lg me-1"></i>
              Ajouter un compte
            </button>
          </div>
        </div>
      </nav>
      <div className="container-lg bg-primary-subtle"></div>
    </>
  );
}

export default Nav;
