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

  interface DateAvailability {
    date: string
    status: 'available' | 'full' | 'weekend'
    appointments_count: number
    max_appointments: number
  }

  interface AvailabilityResponse {
    doctor_id: string
    dates: DateAvailability[]
    start_date: string
    end_date: string
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

  const fetchDoctorAvailability = async (doctorId: string, startDate?: string, days: number = 30) => {
    try {
      const params: any = { days }
      if (startDate) {
        params.start_date = startDate
      }

      const response = await $fetch<AvailabilityResponse>(
        `${apiBase}/api/doctors/${doctorId}/available-dates`,
        { params }
      )
      return response
    } catch (error) {
      console.error('Error fetching doctor availability:', error)
      throw error
    }
  }

  return {
    fetchSpecialties,
    fetchDoctorsBySpecialty,
    fetchDoctorAvailability
  }
}
