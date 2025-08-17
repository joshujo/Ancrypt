import { useEffect, useState } from "react";
import "./Styles.css";
import { Button, Modal } from "@mui/material";
import NewFile from "./NewFile";
import { invoke } from "@tauri-apps/api/core";

import OpenVault from "./OpenVault";
import DeleteVault from "./DeleteVault";

type FileName = {
  name: string;
  id: number;
};

export default function Login() {
  const [files, setFiles] = useState<null | FileName[]>(null);
  const [change, setChange] = useState(0);
  const [disclaimer, setDisclaimer] = useState(false);

  useEffect(() => {
    invoke<FileName[]>("request_vaults").then((e) => {
      setFiles(e);
    });

    const interval = setInterval(() => {
      invoke<FileName[]>("request_vaults").then((e) => {
        setFiles(e);
      });
    }, 10000);

    return () => clearInterval(interval);
  }, [change]);

  const handleOpenDisclaimer = () => {
    setDisclaimer(true);
  };

  const handleCloseDisclaimer = () => {
    setDisclaimer(false);
  };

  return (
    <div className="LoginContainerContainer">
      <h1 className="LoginTitle">Ancrypt</h1>
      <div className="LoginContainer">
        {files && files.length > 0 ? (
          files.map((data) => <FileCard key={data.id} name={data.name} id={data.id} change={setChange} />)
        ) : (
          <p>You have no vaults, create a new vault below</p>
        )}
      </div>
      <NewFile setChange={setChange} />
      <Modal open={disclaimer} onClose={handleCloseDisclaimer}>
        <div
        style={{
          height: "600px",
          width: "800px",
          backgroundColor: "#2c2c2cff",
          position: "absolute",
          left: "50%",
          top: "50%",
          transform: "translate(-50%, -50%)",
          borderRadius: "20px",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          padding: "10px",
          overflowY: "auto"
        }}
        >
          <h1>Disclaimer</h1>
          <p>This project is purely for educational purposes. The security afforded by this program is moderate however it lacks certain sercurity features. The primary one is that this program does not yet zeroise the memory once the decrypted data is dropped. This is fine for most purposes however as unless your system is infected with a malware that can read your memory directly, you can consider that a negligible risk. Additionally, this application has not been audited or thoroughly tested by cybersecurity experts so the use of this application inherently holds a risk and so you should probably not store any actual data in it. Steps have been taken to encrypt data using secure algorithms that have been audited, the main risk arises from after the user decrypts the data where it is kept in memory as plaintext.</p>
          <p>This program attempts to mitigate security issues by limiting access to the passwords purely to the backend. The only time when your passwords aren't in memory is when it is in your clipboard. The only time passwords are on the frontend is when you are adding them to the password manager to store. Additionally, your clipboard is automatically cleared after 30 seconds to minimise the risk of other apps or malware copying your clipboard.</p>
          <p>This application is for educational purposes, the composer does not claim any responsibility for any damages from using this software. If you choose to use it for more than demonstration purposes, note that you disavow the software of all responsibility and that you are using this in a purely low risk environment.</p>
          <p><strong>This software is provided "as-is", without warranty of any kind, and the author is not responsible for any damages arising from its use.</strong></p>
        </div>
      </Modal>
      <Button 
      sx={{
        position: "absolute",
        bottom: 0,
        color: "white",
        backgroundColor: "red",
        margin: "5px"
      }}
      onClick={handleOpenDisclaimer}>Disclaimer</Button>
    </div>
  );
}

interface FileCardProps {
  name: string,
  id: number,
  change: React.Dispatch<React.SetStateAction<number>>
}

function FileCard(props: FileCardProps) {
  return (
    <div className="FileCard">
      <h3 className="FileCardName">{props.name}</h3>
      <OpenVault {...props} />
      <DeleteVault change={props.change} id={props.id} name={props.name} />
    </div>
  );
}
