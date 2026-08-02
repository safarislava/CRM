import React, { useState, useMemo } from 'react'
import type { Role } from '../../types'
import {
  useGetAdminUsersQuery,
  useUpdateUserRolesMutation,
  useDeleteUserMutation,
  type AdminUserItem,
} from '../../store/api/adminApi'
import { useGetMeQuery } from '../../store/api/usersApi'
import styles from './AdminPage.module.scss'

export default function AdminUsersSection() {
  const { data: users = [], isLoading } = useGetAdminUsersQuery()
  const { data: me } = useGetMeQuery()
  const [updateUserRoles, { isLoading: updatingRoles }] = useUpdateUserRolesMutation()
  const [deleteUser, { isLoading: deletingUser }] = useDeleteUserMutation()

  const [search, setSearch] = useState('')
  const [editingUser, setEditingUser] = useState<AdminUserItem | null>(null)
  const [deletingUserItem, setDeletingUserItem] = useState<AdminUserItem | null>(null)
  const [selectedRoles, setSelectedRoles] = useState<Role[]>([])

  const filteredUsers = useMemo(() => {
    return users.filter(
      (u) =>
        u.username.toLowerCase().includes(search.toLowerCase()) ||
        u.email.toLowerCase().includes(search.toLowerCase()),
    )
  }, [users, search])

  const openRolesModal = (user: AdminUserItem) => {
    setEditingUser(user)
    setSelectedRoles([...user.roles])
  }

  const toggleRole = (role: Role) => {
    setSelectedRoles((prev) =>
      prev.includes(role) ? prev.filter((r) => r !== role) : [...prev, role],
    )
  }

  const handleSaveRoles = async () => {
    if (!editingUser) return
    await updateUserRoles({ userId: editingUser.id, roles: selectedRoles })
    setEditingUser(null)
  }

  const handleConfirmDelete = async () => {
    if (!deletingUserItem) return
    await deleteUser({ userId: deletingUserItem.id })
    setDeletingUserItem(null)
  }

  const roleLabels: Record<Role, string> = {
    admin: 'Администратор',
    gip: 'ГИП',
    lawyer: 'Юрист',
    accountant: 'Бухгалтер',
  }

  const getRoleBadgeClass = (role: Role) => {
    switch (role) {
      case 'admin':
        return styles.badgeAdmin
      case 'gip':
        return styles.badgeGip
      case 'lawyer':
        return styles.badgeLawyer
      case 'accountant':
        return styles.badgeAccountant
    }
  }

  return (
    <div className={styles.section}>
      <div className={styles.tableContainer}>
        <div className={styles.toolbar}>
          <div className={styles.searchBox}>
            <SearchIcon />
            <input
              placeholder="Поиск по имени или email…"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
          <span className={styles.statLabel}>Всего пользователей: {filteredUsers.length}</span>
        </div>

        {isLoading ? (
          <div style={{ padding: 24, textAlign: 'center' }}>Загрузка пользователей…</div>
        ) : (
          <table className={styles.table}>
            <thead>
              <tr>
                <th>Пользователь</th>
                <th>Email</th>
                <th>Роли</th>
                <th>Дата регистрации</th>
                <th style={{ textAlign: 'right' }}>Действия</th>
              </tr>
            </thead>
            <tbody>
              {filteredUsers.map((user) => {
                const isSelf = user.username === me?.username
                return (
                  <tr key={user.id}>
                    <td>
                      <strong>{user.username}</strong>
                      {isSelf && (
                        <span style={{ marginLeft: 8, fontSize: '0.75rem', opacity: 0.7 }}>
                          (Вы)
                        </span>
                      )}
                    </td>
                    <td>{user.email || '—'}</td>
                    <td>
                      <div className={styles.rolesList}>
                        {user.roles.length === 0 ? (
                          <span style={{ opacity: 0.5, fontSize: '0.8rem' }}>Без роли</span>
                        ) : (
                          user.roles.map((r) => (
                            <span key={r} className={`${styles.roleBadge} ${getRoleBadgeClass(r)}`}>
                              {roleLabels[r]}
                            </span>
                          ))
                        )}
                      </div>
                    </td>
                    <td>{new Date(user.created_at).toLocaleDateString('ru-RU')}</td>
                    <td style={{ textAlign: 'right' }}>
                      <button
                        className={styles.iconBtn}
                        onClick={() => openRolesModal(user)}
                        title="Изменить роли"
                      >
                        <EditIcon />
                      </button>
                      {!isSelf && (
                        <button
                          className={`${styles.iconBtn} ${styles.deleteBtn}`}
                          onClick={() => setDeletingUserItem(user)}
                          title="Удалить пользователя"
                        >
                          <TrashIcon />
                        </button>
                      )}
                    </td>
                  </tr>
                )
              })}
            </tbody>
          </table>
        )}
      </div>

      {/* Role Edit Modal */}
      {editingUser && (
        <div className={styles.modalBackdrop} onClick={() => setEditingUser(null)}>
          <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
            <h3 className={styles.modalTitle}>
              Редактирование ролей: {editingUser.username}
            </h3>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 12, margin: '16px 0' }}>
              {(['admin', 'gip', 'lawyer', 'accountant'] as Role[]).map((role) => {
                const active = selectedRoles.includes(role)
                return (
                  <label
                    key={role}
                    style={{
                      display: 'flex',
                      alignItems: 'center',
                      gap: 12,
                      padding: '10px 14px',
                      borderRadius: 8,
                      background: active ? 'rgba(99, 102, 241, 0.15)' : 'rgba(255,255,255,0.03)',
                      border: active ? '1px solid #6366f1' : '1px solid transparent',
                      cursor: 'pointer',
                    }}
                  >
                    <input
                      type="checkbox"
                      checked={active}
                      onChange={() => toggleRole(role)}
                    />
                    <span style={{ fontWeight: 600 }}>{roleLabels[role]}</span>
                  </label>
                )
              })}
            </div>
            <div className={styles.modalActions}>
              <button className={styles.cancelBtn} onClick={() => setEditingUser(null)}>
                Отмена
              </button>
              <button
                className={styles.actionBtn}
                onClick={handleSaveRoles}
                disabled={updatingRoles}
              >
                Сохранить
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Delete User Modal */}
      {deletingUserItem && (
        <div className={styles.modalBackdrop} onClick={() => setDeletingUserItem(null)}>
          <div className={styles.modal} onClick={(e) => e.stopPropagation()}>
            <h3 className={styles.modalTitle}>Удаление пользователя</h3>
            <p className={styles.statLabel}>
              Вы действительно хотите удалить пользователя <strong>{deletingUserItem.username}</strong>? Это действие неперевершимо.
            </p>
            <div className={styles.modalActions}>
              <button className={styles.cancelBtn} onClick={() => setDeletingUserItem(null)}>
                Отмена
              </button>
              <button
                className={styles.actionBtn}
                style={{ background: '#ef4444' }}
                onClick={handleConfirmDelete}
                disabled={deletingUser}
              >
                Удалить
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

function SearchIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
      <circle cx="11" cy="11" r="7" stroke="currentColor" strokeWidth="2" />
      <path d="m16.5 16.5 4 4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function EditIcon() {
  return (
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none">
      <path d="M12 20h9" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <path d="M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
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
