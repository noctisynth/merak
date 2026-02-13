import { LayoutDashboard, Settings } from 'lucide-react';
import { Button } from '@/components/ui/button';

export function DashboardSidebar() {
  return (
    <aside className="hidden w-64 flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground md:flex">
      <div className="p-6 text-lg font-semibold">Merak</div>

      <nav className="flex flex-col gap-2 px-4">
        <Button
          variant="ghost"
          className="justify-start text-sidebar-foreground hover:bg-sidebar-accent"
        >
          <LayoutDashboard className="mr-2 h-4 w-4" />
          Dashboard
        </Button>

        <Button
          variant="ghost"
          className="justify-start text-sidebar-foreground hover:bg-sidebar-accent"
        >
          <Settings className="mr-2 h-4 w-4" />
          Settings
        </Button>
      </nav>
    </aside>
  );
}
