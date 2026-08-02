import React, { useState } from 'react'
import {
  useGetAdminInvitationsQuery,
  useCreateInvitationMutation,
  useRevokeInvitationMutation,
} from '../../store/api/adminApi'
import styles from './AdminPage.module.scss'

export default function AdminInvitationsSection() {
  const { data: invitations = [], isLoading } = useGetAdminInvitationsQuery()
  const [createInvitation, { isLoading: creating }] = useCreateInvitationMutation()
  const [revokeInvitation, { isLoading: revoking }] = useRevokeInvitationMutation()
  const [copiedToken, setCopiedToken] = useState<string | null>(null)

  const handleCreate = async () => {
    await createInvitation()
  }

  const handleCopyLink = (token: string) => {
    const link = `${window.location.origin}/?invite=${token}`
    navigator.clipboard.writeText(link)
    setCopiedToken(token)
    setTimeout(() => setCopiedToken(null), 2500)
  }

  const handleRevoke = async (token: string) => {
    await revokeInvitation({ token })
  }

  return (
    <div className={styles.section}>
      <div className={styles.tableContainer}>
        <div className={styles.toolbar}>
          <button
            className={styles.actionBtn}
            onClick={handleCreate}
            disabled={creating}
          >
            <PlusIcon />
            Создать приглашение
          </button>
          <span className={styles.statLabel}>
            Активных приглашений: {invitations.length}
          </span>
        </div>

        {isLoading ? (
          <div style={{ padding: 24, textAlign: 'center' }}>Загрузка приглашений…</div>
        ) : invitations.length === 0 ? (
          <div style={{ padding: 32, textAlign: 'center', color: '#94a3b8' }}>
            Нет активных приглашений. Нажмите «Создать приглашение», чтобы сгенерировать ссылку.
          </div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th>Токен приглашения</th>
                <th>Дата создания</th>
                <th>Действителен до</th>
                <th style={{ textAlign: 'right' }}>Действия</th>
              </tr>
            </thead>
            <tbody>
              {invitations.map((inv) => {
                const isCopied = copiedToken === inv.token
                return (
                  <tr key={inv.token}>
                    <td style={{ fontFamily: 'monospace' }}>{inv.token}</td>
                    <td>{new Date(inv.created_at).toLocaleString('ru-RU')}</td>
                    <td>{new Date(inv.expires_at).toLocaleString('ru-RU')}</td>
                    <td style={{ textAlign: 'right', display: 'flex', gap: 8, justifyContent: 'flex-end' }}>
                      <button
                        className={styles.actionBtn}
                        style={{
                          background: isCopied ? '#10b981' : 'rgba(99, 102, 241, 0.2)',
                          color: isCopied ? '#ffffff' : '#818cf8',
                        }}
                        onClick={() => handleCopyLink(inv.token)}
                      >
                        {isCopied ? 'Ссылка скопирована!' : 'Скопировать ссылку'}
                      </button>
                      <button
                        className={`${styles.iconBtn} ${styles.deleteBtn}`}
                        onClick={() => handleRevoke(inv.token)}
                        disabled={revoking}
                        title="Отозвать приглашение"
                      >
                        <TrashIcon />
                      </button>
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        )}
      </div>
    </div>
  )
}

function PlusIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
      <line x1="12" y1="5" x2="12" y2="19" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <line x1="5" y1="12" x2="19" y2="12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function TrashIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
      <polyline points="3 6 5 6 21 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}
