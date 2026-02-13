import { Button } from '@/components/ui/button';

export function DashboardHeader() {
  return (
    <header className="flex h-14 items-center justify-between border-b border-border bg-background px-6">
      <h1 className="text-lg font-semibold">Dashboard</h1>

      <Button size="sm">Action</Button>
    </header>
  );
}
