export interface LoginRequest {
  identifier: string;
  password: string;
}

export async function login(payload: LoginRequest) {
  const res = await fetch('http://127.0.0.1:8080/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    throw new Error(`Login failed: ${res.status}`);
  }

  return res.json();
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export async function register(payload: RegisterRequest) {
  const res = await fetch('http://127.0.0.1:8080/auth/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    throw new Error(`Register failed: ${res.status}`);
  }

  return res.json();
}
