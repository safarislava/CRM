import React, { useState } from 'react'
import type { Role } from '../../../types'
import { useUpdateRolesMutation } from '../../../store/crmApi'
import styles from '../UserPage.module.scss'

interface UserRolesSectionProps {
  currentRoles: Role[]
}

export default function UserRolesSection({ currentRoles }: UserRolesSectionProps) {
  const [rolesSuccess, setRolesSuccess] = useState(false)
  const [updateRoles, { isLoading: savingRoles }] = useUpdateRolesMutation()

  const handleRoleToggle = async (role: Role) => {
    setRolesSuccess(false)
    const current = currentRoles ?? []
    const next = current.includes(role) ? current.filter((r) => r !== role) : [...current, role]
    const result = await updateRoles({ roles: next })
    if (!('error' in result)) setRolesSuccess(true)
  }

  const labels: Record<Role, string> = {
    gip: 'ГИП',
    lawyer: 'Юрист',
    accountant: 'Бухгалтер',
    admin: 'Администратор',
  }

  return (
    <section className={styles.section}>
      <h2 className={styles.sectionTitle}>Роли</h2>
      <p className={styles.current}>Выберите одну или несколько ролей</p>
      <div className={styles.roles}>
        {(['gip', 'lawyer', 'accountant'] as Role[]).map((role) => {
          const active = (currentRoles ?? []).includes(role)
          return (
            <button
              key={role}
              className={`${styles.roleBtn} ${active ? styles.roleBtnActive : ''}`}
              onClick={() => handleRoleToggle(role)}
              disabled={savingRoles}
            >
              {labels[role]}
            </button>
          )
        })}
      </div>
      {rolesSuccess && <p className={styles.success}>Роли обновлены</p>}
    </section>
  )
}
