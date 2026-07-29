import React, { useState } from 'react'
import { useDispatch } from 'react-redux'
import type { AppDispatch } from '../../store'
import { setUserPageOpen } from '../../store/uiSlice'
import { useGetMeQuery, useLogoutApiMutation } from '../../store/crmApi'
import UserModals from './components/UserModals'
import UserRolesSection from './components/UserRolesSection'
import UserPreferencesSection from './components/UserPreferencesSection'
import UserInvitesSection from './components/UserInvitesSection'
import styles from './UserPage.module.scss'

type ModalKind = 'username' | 'email' | 'password'

export default function UserPage() {
  const dispatch = useDispatch<AppDispatch>()
  const { data: me } = useGetMeQuery()
  const [modal, setModal] = useState<ModalKind | null>(null)
  const [logoutApi] = useLogoutApiMutation()

  return (
    <div className={styles.page}>
      <UserModals modal={modal} onClose={() => setModal(null)} />

      <header className={styles.header}>
        <button className={styles.back} onClick={() => dispatch(setUserPageOpen(false))}>
          <BackIcon />
        </button>
        <h1 className={styles.title}>Профиль</h1>
      </header>

      <div className={styles.content}>
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>Имя пользователя</h2>
            <button className={styles.editBtn} onClick={() => setModal('username')}>
              Изменить
            </button>
          </div>
          {me && (
            <p className={styles.current}>
              Текущее: <strong>{me.username}</strong>
            </p>
          )}
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>Email</h2>
            <button className={styles.editBtn} onClick={() => setModal('email')}>
              Изменить
            </button>
          </div>
          {me && (
            <p className={styles.current}>
              Текущий: <strong>{me.email}</strong>
            </p>
          )}
        </section>

        <div className={styles.divider} />

        {me && <UserRolesSection currentRoles={me.roles ?? []} />}

        <div className={styles.divider} />

        {me && <UserPreferencesSection notificationsEnabled={me.notifications_enabled} />}

        <div className={styles.divider} />

        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>Пароль</h2>
            <button className={styles.editBtn} onClick={() => setModal('password')}>
              Изменить
            </button>
          </div>
        </section>

        <div className={styles.divider} />

        <UserInvitesSection />

        <div className={styles.divider} />

        <section className={styles.section}>
          <button className={`${styles.btn} ${styles.btnDanger}`} onClick={() => logoutApi()}>
            Выйти из аккаунта
          </button>
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Техническая поддержка</h2>
          <a className={styles.support} href="mailto:safarislava@gmail.com">
            safarislava@gmail.com
          </a>
        </section>
      </div>
    </div>
  )
}

function BackIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path
        d="M19 12H5M12 5l-7 7 7 7"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}