import { useNavigate } from "react-router-dom";
import "./App.scss";

export default function AddAsset() {
  const navigate = useNavigate();
  return (
    <div className="bg-light bg-gradient container-md flex-start p-4">
      <div className="d-flex flex-row justify-content-between mb-3">
        <h2 className="p-2">Completer mon patrimoine</h2>
        <button onClick={() => navigate("/")}>Retour</button>
      </div>
      <input
        type="text"
        className="form-control p-2 mt-3"
        placeholder="BoursoBank, Bourse Direct, ..."
      />
    </div>
  );
}
