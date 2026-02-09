import { useState } from 'react';
import { Link, useNavigate } from 'react-router';
import { login } from '@/client';

export default function Login() {
  const navigate = useNavigate();
  const [identifier, setIdentifier] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async () => {
    try {
      setLoading(true);
      setError('');

      await login({
        body: {
          identifier,
          password,
        },
      });

      navigate('/register');
    } catch {
      setError('Invalid email or password');
    } finally {
      setLoading(false);
    }
  };
  return (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <div className="w-full max-w-md rounded-xl border bg-card shadow-sm">
        <div className="flex items-center justify-between p-6 pb-4">
          <div>
            <h1 className="text-xl font-semibold text-foreground">
              Login to your account
            </h1>
            <p className="text-sm text-muted-foreground mt-1">
              Enter your email below to login to your account
            </p>
          </div>

          <Link
            to="/register"
            className="text-sm font-medium text-foreground hover:underline"
          >
            Sign Up
          </Link>
        </div>

        <div className="px-6 pb-6 space-y-4">
          <div className="space-y-2">
            <label
              htmlFor="login-email"
              className="text-sm font-medium text-foreground"
            >
              Email
            </label>
            <input
              id="login-email"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              placeholder="m@example.com"
              value={identifier}
              onChange={(e) => setIdentifier(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <label
                htmlFor="login-password"
                className="text-sm font-medium text-foreground"
              >
                Password
              </label>
              <span className="text-sm text-muted-foreground cursor-pointer">
                Forgot your password?
              </span>
            </div>
            <input
              id="login-password"
              type="password"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              placeholder="••••••••"
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
            {loading ? 'Logging in...' : 'Login'}
          </button>
        </div>
      </div>
    </div>
  );
}
