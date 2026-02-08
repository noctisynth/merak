import { useState } from 'react';
import { login } from '@/services/auth';

export default function Login() {
  const [identifier, setIdentifier] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  async function handleSubmit() {
    try {
      setLoading(true);
      setError('');
      const data = await login({
        identifier,
        password,
      });

      console.log('login success:', data);
    } catch (err) {
      console.error(err);
      alert('Login fail');
      setError('Login fail');
    } finally {
      setLoading(false);
    }
  }
  return (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <div className="w-full max-w-sm rounded-lg border bg-card p-6 shadow-sm">
        <h1 className="mb-4 text-xl font-semibold text-foreground">Login</h1>

        <input
          className="mb-2 w-full rounded-md border bg-background p-2 text-foreground"
          placeholder="Username or Email"
          value={identifier}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            setIdentifier(e.target.value)
          }
        />

        <input
          className="mb-2 w-full rounded-md border bg-background p-2 text-foreground"
          placeholder="Password"
          type="password"
          value={password}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            setPassword(e.target.value)
          }
        />

        {error && <p className="mb-2 text-sm text-destructive">{error}</p>}

        <button
          type="button"
          className="w-full rounded-md border px-4 py-2 text-sm text-foreground hover:bg-muted disabled:opacity-50"
          onClick={handleSubmit}
          disabled={loading}
        >
          {loading ? 'Logging in...' : 'Login'}
        </button>
      </div>
    </div>
  );
}
