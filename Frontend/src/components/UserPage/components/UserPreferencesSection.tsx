import React from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../../store'
import { setTheme, type Theme } from '../../../store/uiSlice'
import { useUpdateNotificationsMutation } from '../../../store/crmApi'
import styles from '../UserPage.module.scss'

interface UserPreferencesSectionProps {
  notificationsEnabled: boolean
}

export default function UserPreferencesSection({ notificationsEnabled }: UserPreferencesSectionProps) {
  const dispatch = useDispatch<AppDispatch>()
  const theme = useSelector((s: RootState) => s.ui.theme)
  const [updateNotifications, { isLoading: savingNotifications }] = useUpdateNotificationsMutation()

  return (
    <>
      <section className={styles.section}>
        <h2 className={styles.sectionTitle}>Уведомления</h2>
        <p className={styles.current}>Письма о дедлайнах на следующий день</p>
        <button
          className={`${styles.btn} ${notificationsEnabled ? styles.btnDanger : ''}`}
          onClick={() => updateNotifications({ enabled: !notificationsEnabled })}
          disabled={savingNotifications}
        >
          {savingNotifications ? '…' : notificationsEnabled ? 'Отключить' : 'Включить'}
        </button>
      </section>

      <div className={styles.divider} />

      <section className={styles.section}>
        <h2 className={styles.sectionTitle}>Тема</h2>
        <div className={styles.themeSwitch}>
          {(['dark', 'auto', 'light'] as Theme[]).map((t) => (
            <button
              key={t}
              className={`${styles.themeBtn} ${theme === t ? styles.themeBtnActive : ''}`}
              onClick={() => dispatch(setTheme(t))}
            >
              {t === 'dark' && <MoonIcon />}
              {t === 'auto' && <MonitorIcon />}
              {t === 'light' && <SunIcon />}
              {t === 'dark' ? 'Тёмная' : t === 'auto' ? 'Авто' : 'Светлая'}
            </button>
          ))}
        </div>
      </section>
    </>
  )
}

function MoonIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none">
      <path
        d="M21 12.79A9 9 0 1 1 11.21 3a7 7 0 0 0 9.79 9.79Z"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}

function SunIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="5" stroke="currentColor" strokeWidth="2" />
      <line x1="12" y1="2" x2="12" y2="4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="12" y1="20" x2="12" y2="22" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="2" y1="12" x2="4" y2="12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="20" y1="12" x2="22" y2="12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function MonitorIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none">
      <rect x="2" y="3" width="20" height="14" rx="2" stroke="currentColor" strokeWidth="2" />
      <line x1="8" y1="21" x2="16" y2="21" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="12" y1="17" x2="12" y2="21" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}
