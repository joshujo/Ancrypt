import { Button, createTheme, TextField, ThemeProvider } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./Vault.css";
import { useNavigate } from "react-router";

export default function Vault() {
  const [passwordList, setPasswordList] = useState<null | string[]>(null);
  const [change, setChange] = useState(0);
  const [newPassword, setNewPassword] = useState("");
  const [newName, setNewName] = useState("");

  const navigate = useNavigate();

  const triggerChange = () => {
    setChange((prev) => prev + 1);
  };

  const handleNewPassword = (e: React.ChangeEvent<HTMLInputElement>) => {
    setNewPassword(e.target.value);
  };

  const handleNewName = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.value.length > 40) {
        return
    }
    setNewName(e.target.value)
  }

  const handleLock = () => {
    invoke("lock_vault").then(() => {
        setPasswordList(null);
        setNewPassword("");
        setNewName("");
        navigate("/");
    })
  }

  const handlePasswordSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    invoke("add_password", {
        name: newName,
        password: newPassword
    }).then(() => {
        setNewPassword("");
        setNewName("");
        triggerChange()
    }).catch((e) => {
        console.log(e)
    })
  }

  useEffect(() => {
    invoke<string[]>("retrieve_password_list").then((e) => {
      setPasswordList(e);
    });
  }, [change]);

  const theme = createTheme({
    components: {
      MuiTextField: {
        styleOverrides: {
          root: {
            input: {
              color: "white",
            },
            label: {
              color: "#cacacaff",
            },
            "& .MuiOutlinedInput-root": {
              backgroundColor: "#4b4b4bff",
            },
            "& .MuiOutlinedInput-root.Mui-focused .MuiOutlinedInput-notchedOutline":
              {
                borderColor: "white",
                color: "white",
              },
            "& .MuiInputLabel-root.Mui-focused": {
              color: "white",
            },
            width: "30vw",
          },
        },
      },
    },
  });

  return (
    <div>
      <h1>Vault</h1>
      <div className="VaultContainer">
        <div className="VaultSideContainer">
          <div className="PasswordPanelContainer">
            {passwordList && passwordList.length > 0 ? (
              passwordList.map((data, index) => <PasswordPanel key={index} password={data} />)
            ) : (
              <p>No passwords yet, create some passwords to store</p>
            )}
          </div>
        </div>
        <div className="VaultSideContainer">
          <div className="NewPasswordForm">
            <h1>Add Password</h1>
            <form className="NewPasswordForm"
            onSubmit={handlePasswordSubmit}
            >
              <ThemeProvider theme={theme}>
                <TextField
                  value={newName}
                  onChange={handleNewName}
                  autoFocus={true}
                  autoComplete="off"
                  label="Password Name"
                />
                <TextField
                value={newPassword}
                onChange={handleNewPassword}
                autoComplete="off"
                type="password"
                label="Password"
                />
                <Button type="submit"
                sx={{
                    backgroundColor: "green",
                    color: "white"
                }}
                >Submit</Button>
              </ThemeProvider>
            </form>
          </div>
          <p
        style={{
            color: "red"
        }}
        >Clipboard is automatically cleared after 30 seconds</p>
        <Button
        onClick={handleLock}
        sx={{
            backgroundColor: "#d62222ff",
            color: "white",
            width: "100px"
        }}
        >Lock Vault</Button>
        </div>
      </div>
    </div>
  );
}

interface PasswordPanelProps {
  password: string;
}

function PasswordPanel({ password }: PasswordPanelProps) {

    const copy = () => {
        invoke("copy_to_clipboard", {
            password: password
        });
    }

  return (
    <div className="PasswordPanel">
      {
        password.length < 10 ? (
            <h2>{password}</h2>
        ) : (
            <h4>{password}</h4>
        )
      }
      <Button 
      sx={{
        color: "white",
        backgroundColor: "rgba(0, 128, 255, 0.47)"
      }}
      onClick={copy}>Copy to clipboard</Button>
    </div>
  );
}
