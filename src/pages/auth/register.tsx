import { register } from '@/services/auth';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router';

export default function Register() {
  const navigate = useNavigate();

  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  async function handleSubmit() {
    try {
      setLoading(true);
      setError('');

      await register({
        username,
        email,
        password,
      });

      navigate('/');
    } catch {
      setError('Sign-up failed');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <div className="w-full max-w-md rounded-xl border bg-card shadow-sm">
        <div className="flex items-center justify-between p-6 pb-4">
          <div>
            <h1 className="text-xl font-semibold text-foreground">
              Create an account
            </h1>
            <p className="text-sm text-muted-foreground mt-1">
              Enter your information to create an account
            </p>
          </div>

          <Link
            to="/"
            className="text-sm font-medium text-foreground hover:underline"
          >
            Log in
          </Link>
        </div>

        <div className="px-6 pb-6 space-y-4">
          <div className="space-y-2">
            <label
              htmlFor="register-username"
              className="text-sm font-medium text-foreground"
            >
              Username
            </label>
            <input
              id="register-username"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <label
              htmlFor="email"
              className="text-sm font-medium text-foreground"
            >
              Email
            </label>
            <input
              id="email"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <label
              htmlFor="register-password"
              className="text-sm font-medium text-foreground"
            >
              Password
            </label>
            <input
              id="register-password"
              type="password"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
          </div>

          {error && <p className="text-sm text-destructive">{error}</p>}

          <button
            type="button"
            onClick={handleSubmit}
            disabled={loading}
            className="w-full rounded-md bg-foreground px-4 py-2 text-sm font-medium text-background hover:bg-foreground/90 disabled:opacity-50"
          >
            {loading ? 'Registering...' : 'Register'}
          </button>
        </div>
      </div>
    </div>
  );
}
