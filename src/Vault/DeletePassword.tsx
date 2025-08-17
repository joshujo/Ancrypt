import { Button, createTheme, IconButton, Modal, TextField, ThemeProvider } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { lazy, useState } from "react";

interface props {
    change: React.Dispatch<React.SetStateAction<number>>,
    password: string,
}

const DeleteIcon = lazy(() => import("@mui/icons-material/Delete"));

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
            "& .MuiOutlinedInput-input": {
                textAlign: "center"
            },
            width: "100px",
            textAlign: "center"
          },
        },
      },
    },
  });

export default function DeletePassword({change, password}: props) {
    const [open, setOpen] = useState(false);
    const [code, setCode] = useState(0);
    const [validationCode, setValidationCode] = useState(0);
    const [errorMessage, setErrorMessage] = useState("");

    const handleOpen = () => {
        setOpen(true);
        invoke<number>("five_number_rng").then((e) => {
            setCode(e);
        })
    };

    const handleClose = () => {
        setOpen(false);
    };

    const handleDelete = () => {
        change(prev => prev + 1);

        if (validationCode != code) {
            setErrorMessage("The code you entered was incorrect, try again!");
            return;
        }

        invoke("delete_password", {
            name: password
        });
        setOpen(false);
        setValidationCode(0);
        setErrorMessage("");
    }

    const handleValidationCodeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const numericValue = e.target.value.replace(/[^0-9]/g, '').slice(0, 6);
        let number = parseInt(numericValue);
        if (Number.isNaN(number)) {
            number = 0
        }
        setValidationCode(number);
    }

    return (
        <div>
        <IconButton
        className="FileCardDelete"
        sx={{
          backgroundColor: "rgba(22, 45, 163, 0.53)",
          "&:hover": {
            backgroundColor: "#34b3efff",
          },
        }}
        onClick={handleOpen}
      >
        <DeleteIcon
          sx={{
            color: "white",
          }}
        />
      </IconButton>
      <Modal open={open}
      onClose={handleClose}
      >
          <div className="DeleteContainer">
            <h1>Delete Password</h1>
            <p>Are you sure you want to delete password: <strong>{password}</strong>?</p>
            <p>Enter the code: <strong>{code}</strong></p>
            <ThemeProvider theme={theme}>
            <TextField 
            slotProps={{
                input: {
                    inputMode: "numeric"
                }
            }}
            value={validationCode}
            onChange={handleValidationCodeChange}
            />
            </ThemeProvider>
            <Button
            sx={{
                backgroundColor: "red",
                color: "white",
                padding: "10px",
                margin: "10px"
            }}
            onClick={handleDelete}
            >Confirm Delete</Button>
            <p
            style={{
                color: "red"
            }}
            >{errorMessage}</p>
          </div>
      </Modal>
      </div>
    )
}