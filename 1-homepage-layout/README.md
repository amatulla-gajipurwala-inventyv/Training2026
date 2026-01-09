# Homepage Layout – Authentication UI

This project is a responsive authentication interface built using **HTML and CSS only**, without JavaScript.  
It provides **Login**, **Sign Up**, and **Forgot Password** forms using a clean 70–30 split layout.

The page uses **CSS radio-button toggling** to switch between forms, demonstrating logic-driven UI behavior with minimal code.

---

## Features

- Login, Sign Up, and Forgot Password forms
- Pure HTML and CSS (no JavaScript)
- Radio-button based form toggling
- 70% company information panel and 30% authentication panel
- Fully responsive design for desktop, tablet, and mobile
- Clean and modern UI with smooth transitions

---

## Layout Structure

- **Left Section (70%)**
  - Company logo
  - Company name
  - Company description
  - Gradient background

- **Right Section (30%)**
  - Login form
  - Sign Up form
  - Forgot Password form
  - Toggle navigation using radio inputs

---

## Technologies Used

- HTML5
- CSS3
  - Flexbox
  - Media Queries
  - Pseudo-elements
  - CSS Transitions

---

## How the UI Works

- Radio inputs (`loginRadio`, `signupRadio`, `fpRadio`) control which form is visible
- CSS sibling selectors (`~`) are used to show or hide forms
- Forms switch dynamically based on selected radio button

---

