import { Button, createTheme, TextField, ThemeProvider } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./Vault.css";
import { useNavigate } from "react-router";
import DeletePassword from "./DeletePassword";

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
      return;
    }
    setNewName(e.target.value);
  };

  const handleLock = () => {
    invoke("lock_vault").then(() => {
      setPasswordList(null);
      setNewPassword("");
      setNewName("");
      navigate("/");
    });
  };

  const handleClearClipboard = () => {
    invoke("clear_clipboard");
  };

  const handlePasswordSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    invoke("add_password", {
      name: newName,
      password: newPassword,
    })
      .then(() => {
        setNewPassword("");
        setNewName("");
        triggerChange();
      })
      .catch((e) => {
        console.log(e);
      });
  };

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
              passwordList.map((data, index) => (
                <PasswordPanel key={index} password={data} change={setChange} />
              ))
            ) : (
              <p>No passwords yet, create some passwords to store</p>
            )}
          </div>
        </div>
        <div className="VaultSideContainer">
          <div className="NewPasswordForm">
            <h1>Add Password</h1>
            <form className="NewPasswordForm" onSubmit={handlePasswordSubmit}>
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
                <Button
                  type="submit"
                  sx={{
                    backgroundColor: "green",
                    color: "white",
                  }}
                >
                  Submit
                </Button>
              </ThemeProvider>
            </form>
          </div>
          <GeneratePassword change={setChange}/>
          <p
            style={{
              color: "red",
            }}
          >
            Clipboard is automatically cleared after 30 seconds
          </p>
          <Button
            onClick={handleLock}
            sx={{
              backgroundColor: "#d62222ff",
              color: "white",
              width: "100px",
            }}
          >
            Lock Vault
          </Button>
          <Button
            onClick={handleClearClipboard}
            sx={{
              backgroundColor: "#292929ff",
              border: "1px solid white",
              color: "white",
              width: "100px",
              marginTop: "10px",
            }}
          >
            Clear Clipboard
          </Button>
        </div>
      </div>
    </div>
  );
}

interface PasswordPanelProps {
  password: string;
  change: React.Dispatch<React.SetStateAction<number>>;
}

function PasswordPanel({ password, change }: PasswordPanelProps) {
  const copy = () => {
    invoke("copy_to_clipboard", {
      password: password,
    });
  };

  return (
    <div className="PasswordPanel">
      {password.length < 10 ? <h2>{password}</h2> : <h4>{password}</h4>}
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          gap: "10px",
        }}
      >
        <Button
          sx={{
            color: "white",
            backgroundColor: "rgba(0, 128, 255, 0.47)",
          }}
          onClick={copy}
        >
          Copy to clipboard
        </Button>
        <DeletePassword change={change} password={password} />
      </div>
    </div>
  );
}

type GeneratePasswordChange = {
  change: React.Dispatch<React.SetStateAction<number>>
}

function GeneratePassword({change}: GeneratePasswordChange) {
  const [name, setName] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    invoke("generate_password", {
      name: name
    }).then(() => {
      change(prev => prev + 1);
      setErrorMessage("");
      setName("");
    }).catch((e: string) => {
      setErrorMessage(e);
    })
  }

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
    <div className="GeneratePassword">
      <h1>Generate New Password</h1>
      <p>
        This will generate a random alphanumeric 18 character UTF-8 password
      </p>
      <ThemeProvider theme={theme}>
        <form className="GeneratePassword"
        onSubmit={handleSubmit}
        >
          <TextField
            label="Password name"
            value={name}
            onChange={handleNameChange}
          />
          <Button
          sx={{
            color: "white",
            backgroundColor: "green"
          }}
          type="submit"
          >
            Generate
          </Button>
        </form>
        </ThemeProvider>
        <p
        style={{
          color: "red"
        }}
        >{errorMessage}</p>
    </div>
  );
}
