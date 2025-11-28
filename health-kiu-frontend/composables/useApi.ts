export const useApi = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase

  interface Specialty {
    id: number
    name: string
    description?: string
  }

  interface Doctor {
    id: string
    name: string
    specialty_id: number
  }

  interface PagedResponse<T> {
    data: T[]
    total: number
    page: number
    page_size: number
  }

  const fetchSpecialties = async (offset: number = 0, count: number = 20) => {
    try {
      const response = await $fetch<PagedResponse<Specialty>>(
        `${apiBase}/api/specialties`,
        {
          params: { offset, count },
          // Disable SSL verification for development with self-signed certs
          // @ts-ignore
          https: { rejectUnauthorized: false }
        }
      )
      return response
    } catch (error) {
      console.error('Error fetching specialties:', error)
      throw error
    }
  }

  const fetchDoctorsBySpecialty = async (specialtyId: number, offset: number = 0, count: number = 20) => {
    try {
      const response = await $fetch<{ specialty: Specialty, data: Doctor[], total: number }>(
        `${apiBase}/api/specialties/${specialtyId}/doctors`,
        {
          params: { offset, count },
          // @ts-ignore
          https: { rejectUnauthorized: false }
        }
      )
      return response
    } catch (error) {
      console.error('Error fetching doctors:', error)
      throw error
    }
  }

  return {
    fetchSpecialties,
    fetchDoctorsBySpecialty
  }
}
