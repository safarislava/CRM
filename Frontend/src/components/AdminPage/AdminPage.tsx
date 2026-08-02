import React, { useState } from 'react'
import { useDispatch } from 'react-redux'
import type { AppDispatch } from '../../store'
import { setAdminPageOpen } from '../../store/uiSlice'
import AdminStatisticsSection from './AdminStatisticsSection'
import AdminUsersSection from './AdminUsersSection'
import AdminInvitationsSection from './AdminInvitationsSection'
import AdminLogsSection from './AdminLogsSection'
import AdminActionsSection from './AdminActionsSection'
import styles from './AdminPage.module.scss'

type AdminTab = 'stats' | 'users' | 'invitations' | 'logs' | 'actions'

export default function AdminPage() {
  const dispatch = useDispatch<AppDispatch>()
  const [activeTab, setActiveTab] = useState<AdminTab>('stats')

  return (
    <div className={styles.container}>
      <header className={styles.header}>
        <div className={styles.titleGroup}>
          <ShieldIcon className={styles.shieldIcon} />
          <div>
            <h1 className={styles.title}>Панель администратора</h1>
            <p className={styles.subtitle}>
              Управление пользователями, ролями, приглашениями и системными логами
            </p>
          </div>
        </div>
        <button
          className={styles.closeBtn}
          onClick={() => dispatch(setAdminPageOpen(false))}
          title="Закрыть панель управления"
        >
          <CloseIcon />
        </button>
      </header>

      <nav className={styles.tabs}>
        <button
          className={`${styles.tabBtn} ${activeTab === 'stats' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('stats')}
        >
          <BarChartIcon />
          Обзор
        </button>

        <button
          className={`${styles.tabBtn} ${activeTab === 'users' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('users')}
        >
          <UsersGroupIcon />
          Пользователи
        </button>

        <button
          className={`${styles.tabBtn} ${activeTab === 'invitations' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('invitations')}
        >
          <MailIcon />
          Приглашения
        </button>

        <button
          className={`${styles.tabBtn} ${activeTab === 'logs' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('logs')}
        >
          <TerminalIcon />
          Логи системы
        </button>

        <button
          className={`${styles.tabBtn} ${activeTab === 'actions' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('actions')}
        >
          <ZapIcon />
          Действия
        </button>
      </nav>

      <main className={styles.contentPane}>
        {activeTab === 'stats' && <AdminStatisticsSection />}
        {activeTab === 'users' && <AdminUsersSection />}
        {activeTab === 'invitations' && <AdminInvitationsSection />}
        {activeTab === 'logs' && <AdminLogsSection />}
        {activeTab === 'actions' && <AdminActionsSection />}
      </main>
    </div>
  )
}

function ShieldIcon({ className }: { className?: string }) {
  return (
    <svg className={className} width="32" height="32" viewBox="0 0 24 24" fill="none">
      <path
        d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}

function CloseIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M18 6 6 18M6 6l12 12" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" />
    </svg>
  )
}

function BarChartIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <line x1="18" y1="20" x2="18" y2="10" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="12" y1="20" x2="12" y2="4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="6" y1="20" x2="6" y2="14" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function UsersGroupIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <circle cx="9" cy="7" r="4" stroke="currentColor" strokeWidth="2" />
    </svg>
  )
}

function MailIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <polyline points="22,6 12,13 2,6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}

function TerminalIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <polyline points="4 17 10 11 4 5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <line x1="12" y1="19" x2="20" y2="19" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function ZapIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}
