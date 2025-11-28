export const useApi = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase

  // API response types matching the Rust backend
  interface Specialty {
    id: number
    name: string
    img_path: string | null
  }

  interface DoctorName {
    first: string
    last: string
  }

  interface Doctor {
    id: string
    cedula: string
    passport: string | null
    name: DoctorName
  }

  interface PagedResponse<T> {
    page_size: number
    next_offset: number | null
    has_more: boolean
    items: T[]
  }

  interface DoctorsBySpecialtyResponse {
    specialty: Specialty
    page_size: number
    next_offset: number | null
    has_more: boolean
    items: Doctor[]
  }

  const fetchSpecialties = async (offset: number = 0, count: number = 20) => {
    try {
      const response = await $fetch<PagedResponse<Specialty>>(
        `${apiBase}/api/specialties`,
        {
          params: { offset, count }
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
      const response = await $fetch<DoctorsBySpecialtyResponse>(
        `${apiBase}/api/specialties/${specialtyId}/doctors`,
        {
          params: { offset, count }
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
