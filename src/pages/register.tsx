import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { Link, useNavigate } from 'react-router';
import { register as registerUser } from '@/client';

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

type RegisterFormValues = {
  username: string;
  email: string;
  password: string;
};

export default function Register() {
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [serverError, setServerError] = useState('');

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<RegisterFormValues>();

  const onSubmit = async (data: RegisterFormValues) => {
    try {
      setLoading(true);
      setServerError('');

      await registerUser({
        body: {
          username: data.username,
          email: data.email,
          password: data.password,
        },
      });

      navigate('/login');
    } catch {
      setServerError('Sign-up failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-background">
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Create an account</CardTitle>
          <CardDescription>
            Enter your information to create an account
          </CardDescription>
          <CardAction>
            <Link
              to="/login"
              className="text-sm font-medium text-foreground hover:underline"
            >
              Log in
            </Link>
          </CardAction>
        </CardHeader>

        <CardContent>
          <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
            <div className="space-y-2">
              <Label
                htmlFor="register-username"
                className="text-sm font-medium text-foreground"
              >
                Username
              </Label>
              <Input
                id="register-username"
                className="bg-background"
                {...register('username', {
                  required: 'Username is required',
                })}
              />
              {errors.username && (
                <p className="text-sm text-destructive">
                  {errors.username.message}
                </p>
              )}
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
                className="bg-background"
                {...register('email', {
                  required: 'Email is required',
                })}
              />
              {errors.email && (
                <p className="text-sm text-destructive">
                  {errors.email.message}
                </p>
              )}
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
                className="bg-background"
                {...register('password', {
                  required: 'Password is required',
                  minLength: {
                    value: 6,
                    message: 'Password must be at least 6 characters',
                  },
                })}
              />
              {errors.password && (
                <p className="text-sm text-destructive">
                  {errors.password.message}
                </p>
              )}
            </div>

            {serverError && (
              <p className="text-sm text-destructive">{serverError}</p>
            )}

            <Button type="submit" disabled={loading} className="w-full">
              {loading ? 'Registering...' : 'Register'}
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
