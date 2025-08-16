import {
  Button,
  CircularProgress,
  createTheme,
  Modal,
  TextField,
  ThemeProvider,
} from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { useNavigate } from "react-router";

interface OpenVaultProps {
  name: string;
  id: number;
}

type OpenVaultResponse = {
  success: boolean;
  message?: string;
};

export default function OpenVault({ name, id }: OpenVaultProps) {
  const [open, setOpen] = useState(false);
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const handleOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  const handleUnlock = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);
    console.log("Unlocking");
    invoke<OpenVaultResponse>("open_vault", {
      id: id,
      password: password,
    }).then((result) => {
      if (result.success == true) {
        navigate("/Vault")
      } else {
        alert("No work");
      }
      setLoading(false);
    });
  };

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
            width: "40vw",
          },
        },
      },
    },
  });

  return (
    <>
      <Button
        className="FileCardButton"
        onClick={handleOpen}
        sx={{
          color: "white",
          backgroundColor: "rgba(22, 45, 163, 0.53)",
          right: 0,
          position: "relative",
        }}
      >
        Open Vault
      </Button>
      <Modal open={open} onClose={handleClose}>
        <div className="PasswordContainer">
          <h1>Open Vault</h1>
          <p>
            Open up: <strong>{name}</strong>
          </p>
          <ThemeProvider theme={theme}>
            <form
              onSubmit={handleUnlock}
              autoComplete="off"
              className="PasswordForm"
            >
              <TextField
                label="Password"
                autoFocus={true}
                value={password}
                onChange={handlePasswordChange}
                type="password"
              />
              <Button
                type="submit"
                sx={{
                  color: "white",
                  backgroundColor: "green",
                  width: "5vw",
                }}
              >
                Unlock
              </Button>
            </form>
          </ThemeProvider>
        </div>
      </Modal>

      <Modal open={loading}>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            position: "relative",
            height: "100vh",
            width: "100wh",
          }}
        >
          <CircularProgress
            aria-busy="true"
            size={200}
            thickness={2}
            sx={{
              position: "relative",
            }}
          />
        </div>
      </Modal>
    </>
  );
}
