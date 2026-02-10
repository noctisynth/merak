import { useState } from 'react';
import { Link, useNavigate } from 'react-router';
import { register } from '@/client';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

export default function Register() {
  const navigate = useNavigate();

  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async () => {
    try {
      setLoading(true);
      setError('');

      await register({
        body: {
          username,
          email,
          password,
        },
      });

      navigate('/login');
    } catch {
      setError('Sign-up failed');
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
            <Label
              htmlFor="register-username"
              className="text-sm font-medium text-foreground"
            >
              Username
            </Label>
            <Input
              id="register-username"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label
              htmlFor="email"
              className="text-sm font-medium text-foreground"
            >
              Email
            </Label>
            <Input
              id="email"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
            />
          </div>

          <div className="space-y-2">
            <Label
              htmlFor="register-password"
              className="text-sm font-medium text-foreground"
            >
              Password
            </Label>
            <Input
              id="register-password"
              type="password"
              className="w-full rounded-md border bg-background px-3 py-2 text-sm text-foreground"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
          </div>

          {error && <p className="text-sm text-destructive">{error}</p>}

          <Button
            type="button"
            onClick={handleSubmit}
            disabled={loading}
            className="w-full"
          >
            {loading ? 'Registering...' : 'Register'}
          </Button>
        </div>
      </div>
    </div>
  );
}
