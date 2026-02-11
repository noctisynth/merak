import { useState } from 'react';
import { Link, useNavigate } from 'react-router';
import { login } from '@/client';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

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
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Login to your account</CardTitle>
          <CardDescription>
            Enter your email below to login to your account
          </CardDescription>
          <CardAction>
            <Link
              to="/register"
              className="text-sm font-medium text-foreground hover:underline"
            >
              Sign Up
            </Link>
          </CardAction>
        </CardHeader>

        <CardContent>
          <div className="space-y-4">
            <div className="space-y-2">
              <Label
                htmlFor="login-email"
                className="text-sm font-medium text-foreground"
              >
                Email
              </Label>
              <Input
                id="login-email"
                className="bg-background"
                placeholder="m@example.com"
                value={identifier}
                onChange={(e) => setIdentifier(e.target.value)}
              />
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <Label
                  htmlFor="login-password"
                  className="text-sm font-medium text-foreground"
                >
                  Password
                </Label>
                <span className="text-sm text-muted-foreground cursor-pointer">
                  Forgot your password?
                </span>
              </div>
              <Input
                id="login-password"
                type="password"
                className="bg-background"
                placeholder="••••••••"
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
              {loading ? 'Logging in...' : 'Login'}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
