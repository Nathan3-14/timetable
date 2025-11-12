import customtkinter as ctk

def toggle_dark_mode() -> None:
    global is_dark_mode
    if is_dark_mode:
        ctk.set_appearance_mode("light")
    else:
        ctk.set_appearance_mode("dark")
    is_dark_mode = not is_dark_mode

app = ctk.CTk()
app.title("Hello World")
app.geometry("360x640")

is_dark_mode = True
ctk.set_appearance_mode("dark")
ctk.set_default_color_theme("green")

label = ctk.CTkLabel(app, text="Hello World!", font=("Courier New", 20))
label.grid(column=0, row=0)

toggle_dark_mode_button = ctk.CTkButton(app, text="Toggle Dark Mode", command=toggle_dark_mode)
toggle_dark_mode_button.grid(column=3,row=0)
toggle_dark_mode_label = ctk.CTkLabel(app, text="test")
toggle_dark_mode_label.grid(column=3, row=5)


app.mainloop()
