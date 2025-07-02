# 📘 Euler_IVP

This project provides implementations of the **Euler method** for solving **ordinary differential equations (ODEs)** using:

- ✅ Rust (`euler.rs`)
- ✅ Python (`Euler.py`)
- ✅ LaTeX (Analytical solution: `analytical_solution.tex`)

---

## ⚙️ Euler Function: Purpose

Solves an Initial Value Problem (IVP) for an ODE using the **explicit Euler method**.

### 🧾 Function Signature (in Python and Rust style)
```python
euler(f, initial_value, start_time, end_time, n_steps)
```

### 📥 Parameters:

- `f` – A function defining the ODE, f(t, y)
- `initial_value` – Initial condition (e.g., y(0) = 1)
- `start_time` – Starting point of solution (e.g., 0.0)
- `end_time` – Ending point of solution (e.g., 5.0)
- `n_steps` – Number of steps (e.g., 20)

### 🔁 Returns:
- A list (Python) or vector (Rust) of approximated `y` values at each step.

---

## 🧪 Example Usage

### In Python:
```python
def f(t, y):
    return math.cos(t) - y

result = euler(f, 1.0, 0.0, 5.0, 20)
```

### In Rust:
```rust
fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}
```

---

## 📐 Analytical Solution

The analytical solution of the same ODE is included using **LaTeX**:

### Problem:
\[
\frac{dy}{dt} = \cos(t) - y, \quad y(0) = 1
\]

### Files:
| File                       | Description                              |
|----------------------------|------------------------------------------|
| `analytical_solution.tex`  | LaTeX file containing full derivation    |
| `analytical_solution.pdf`  | Compiled PDF showing solution steps      |

You can compile using:

```bash
pdflatex analytical_solution.tex
```

---

## 📁 Repository Structure

```
Euler_IVP/
├── Euler.py                   # Python implementation
├── euler.rs                   # Rust implementation
├── analytical_solution.tex    # LaTeX source for analytical solution
├── analytical_solution.pdf    # PDF output of analytical derivation
├── README.md                  # This file
```

---

## 👨‍🎓 Purpose

- Practice solving ODEs numerically and analytically
- Learn cross-language implementation (Python, Rust, LaTeX)
- Develop numerical intuition and LaTeX documentation skills

---

## 👩‍💻 Author

- **Your Name**: [Your GitHub Profile](https://github.com/yourusername)
- **Course**: Numerical Methods / Differential Equations
- **Date**: June 2025
