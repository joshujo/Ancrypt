import { useEffect, useState } from "react";
import "./Styles.css";
import { Button, IconButton } from "@mui/material";
import NewFile from "./NewFile";
import { invoke } from "@tauri-apps/api/core";

import { lazy } from "react";

const DeleteIcon = lazy(() => import("@mui/icons-material/Delete"));

type FileName = {
  name: string;
  id: number;
};

export default function Login() {
  const [files, setFiles] = useState<null | FileName[]>(null);
  const [change, setChange] = useState(0);

  useEffect(() => {
    invoke<FileName[]>("request_vaults").then((e) => {
      setFiles(e);
    });

    const interval = setInterval(() => {
        invoke<FileName[]>("request_vaults").then((e) => {
            setFiles(e);
        })
    }, 10000);

    return () => clearInterval(interval);
  }, [change]);

  return (
    <div className="LoginContainerContainer">
      <h1 className="LoginTitle">Ancrypt</h1>
      <div className="LoginContainer">
        { files && files.length > 0 ? (
          files.map((data) => <FileCard key={data.id} {...data} />)
        ) : (
          <p>You have no vaults, create a new vault below</p>
        )}
      </div>
      <NewFile setChange={setChange}/>
    </div>
  );
}

function FileCard(props: FileName) {
  return (
    <div className="FileCard">
      <h3 className="FileCardName">{props.name}</h3>
      <Button
        className="FileCardButton"
        sx={{
          color: "white",
          backgroundColor: "rgba(22, 45, 163, 0.53)",
          right: 0,
          position: "relative",
        }}
      >
        Open Vault
      </Button>
      <IconButton
        className="FileCardDelete"
        sx={{
          backgroundColor: "rgba(22, 45, 163, 0.53)",
          "&:hover": {
            backgroundColor: "#34b3efff",
          },
        }}
      >
        <DeleteIcon
        sx={{
            color: "white"
        }}
        />
      </IconButton>
    </div>
  );
}
