import { useState } from 'react';
import { register } from '../../services/auth';

export default function Register() {
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  async function handleSubmit() {
    setLoading(true);
    setError('');

    try {
      const data = await register({
        username,
        email,
        password,
      });

      console.log('register success:', data);
    } catch (err) {
      console.error(err);
      setError('注册失败');
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <div className="w-full max-w-sm rounded-lg border bg-card p-6 shadow-sm">
        <h1 className="mb-4 text-xl font-semibold text-foreground">Register</h1>

        <input
          className="mb-2 w-full rounded-md border bg-background p-2 text-foreground"
          placeholder="Username"
          value={username}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            setUsername(e.target.value)
          }
        />

        <input
          className="mb-2 w-full rounded-md border bg-background p-2 text-foreground"
          placeholder="Email"
          value={email}
          onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
            setEmail(e.target.value)
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
          className="w-full rounded-md border px-4 py-2 text-sm text-foreground hover:bg-muted"
          onClick={handleSubmit}
          disabled={loading}
        >
          {loading ? 'Registering...' : 'Register'}
        </button>
      </div>
    </div>
  );
}
