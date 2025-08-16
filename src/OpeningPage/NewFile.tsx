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

type props = {
  setChange: React.Dispatch<React.SetStateAction<number>>;
};

export default function NewFile({ setChange }: props) {
  const [open, setOpen] = useState(false);
  const [vaultName, setVaultName] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);

  const openNewFile = () => {
    setOpen(true);
  };

  const closeNewFile = () => {
    setOpen(false);
  };

  const handleVaultNameChanage = (e: React.ChangeEvent<HTMLInputElement>) => {
    setVaultName(e.target.value);
  };

  const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(e.target.value);
  };

  interface SubmitResponse {
    success: boolean;
    message?: string;
  }

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);
    invoke<SubmitResponse>("create_vault", {
      vault_name: vaultName,
      vault_password: password,
    }).then((e) => {
      if (e.success) {
        setChange((prev) => prev + 1);
        setVaultName("");
        setPassword("");
        setLoading(false);
        setOpen(false);
      } else {
        alert(e.message);
        setLoading(false);
      }
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
    <div>
      <ThemeProvider theme={theme}>
        <Button
          sx={{
            bottom: "20px",
            position: "absolute",
            backgroundColor: "#23a555ff",
            color: "white",
            transform: "translate(-50%, -50%)",
          }}
          onClick={openNewFile}
        >
          New Vault
        </Button>
        <Modal open={open} onClose={closeNewFile}>
          <form
            onSubmit={handleSubmit}
            autoComplete="off"
            autoFocus={true}
            className="NewVaultContainer"
          >
            <h1
              style={{
                marginBottom: "50px",
              }}
            >
              New Vault
            </h1>
            <TextField
              variant="outlined"
              value={vaultName}
              onChange={handleVaultNameChanage}
              label="Vault name"
            />
            <TextField
              variant="outlined"
              type="password"
              label="Vault password"
              value={password}
              onChange={handlePasswordChange}
            />
            <Button
              type="submit"
              sx={{
                color: "white",
                backgroundColor: "rgba(60, 251, 140, 0.53)",
                right: 0,
                position: "relative",
              }}
            >
              Create new vault
            </Button>
          </form>
        </Modal>
        <Modal open={loading}>
          <>
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
          </>
        </Modal>
      </ThemeProvider>
    </div>
  );
}
