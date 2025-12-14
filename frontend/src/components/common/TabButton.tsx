'use client';

import type { Tab } from '@/types';

type TabButtonProps = {
  tab: Tab;
  label: string;
  activeTab: Tab;
  onClick: (tab: Tab) => void;
};

export function TabButton({ tab, label, activeTab, onClick }: TabButtonProps) {
  const isActive = activeTab === tab;
  const isFirst = tab === 'home';
  const isLast = tab === 'settings';

  return (
    <button
      onClick={() => onClick(tab)}
      className={`flex-1 py-2 text-xs font-medium transition-colors ${
        isActive
          ? 'bg-blue-600 text-white'
          : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
      } ${isFirst ? 'rounded-l-lg' : ''} ${isLast ? 'rounded-r-lg' : ''}`}
    >
      {label}
    </button>
  );
}
