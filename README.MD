# 🌀 Euler Method Solver using Rust

This project implements the **Euler method** to solve a **first-order initial value problem (IVP)** using Rust. It also compares the numerical solution with the analytical one and visualizes the results using plots.

---

## 📘 Problem Statement

We aim to solve the differential equation:
dy/dt = cos(t) - y, with y(0) = 1 on the interval [0, 5]


This is a standard ODE (Ordinary Differential Equation) that we solve using both:
- **Euler's method (numerical)**
- **Analytical solution (exact)**

The goal is to compare both solutions for different step sizes and observe the behavior.

---

## 📐 Parameters Used

| Parameter       | Value          | Description                            |
|-----------------|----------------|----------------------------------------|
| `t0`            | 0.0            | Initial time                           |
| `y0`            | 1.0            | Initial value of y                     |
| `T`             | 5.0            | Final time                             |
| `n`             | 20, 100, 1000  | Number of steps (resolution control)   |
| `h`             | (T - t0)/n     | Step size                              |

---

## ⚙️ Methodology

### ➤ Euler Method
- Uses the formula: `y_{n+1} = y_n + h * f(t_n, y_n)`
- Repeats this update step `n` times.

### ➤ Analytical Comparison
- Analytic solution is computed using the known formula:

y(t) = 0.5 * (cos(t) + sin(t)) + 0.5 * exp(-t)

- Used as a reference to measure the accuracy of the Euler method.

### ➤ Step Sizes: n = 20, 100, 1000
- We test the method at different levels of granularity.
- Larger `n` gives better accuracy, but more computation.

---

## 🧪 Technology Stack

### ➤ Rust
- Core language used for computation and file handling.

### ➤ Cargo
- Rust’s package manager and build system.

### ➤ Plotters Crate
- Used to generate comparison plots between Euler and analytic solutions.

### ➤ Google Colab
- Run and visualize Rust code using Python interop and bash cells.

### ➤ CSV Output
- Numerical data exported to `.csv` files for analysis or visualization.

---

## 📁 Folder Structure

euler_project/
│
├── src/
│ ├── main.rs # Contains logic to solve and plot
│ └── euler.rs # Euler method module
│
├── Cargo.toml # Rust configuration and dependencies
├── euler_solutions.csv # Output of Euler values
├── analytic_solution.csv
├── euler_plot.png # Comparison plot saved as image


---

## ▶️ How to Run (in Google Colab)

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
   source $HOME/.cargo/env

2.Create File Structure

Write main.rs, euler.rs, and Cargo.toml
3.Compile & Run
cargo run --release
4. Download CSV or Plot
.Files will be created: euler_solutions.csv, euler_plot.png



---

## 📁 Folder Structure


### ➤ `Cargo.toml`  
### ➤ `euler_solutions.csv`  
### ➤ `euler_plot.png`  

---

## ▶️ How to Run (in Google Colab)

### ➤ Install Rust  
### ➤ Generate files  
### ➤ Compile & run  
### ➤ Download output  

---

## 📊 Output

### ➤ CSV of numerical solution  
### ➤ Plot of Euler vs Analytical  

 Output
➤ CSV Files
Contain step-by-step values of t, y_euler, y_exact, and error.

Files are created for n = 20, 100, 1000.

➤ Plot
Line plot comparing Euler vs Analytical solution for all n.

Saved as euler_plot.png.

