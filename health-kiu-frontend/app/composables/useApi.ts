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
        `${apiBase}/specialty/specialties`,
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
        `${apiBase}/doctor/by_specialty/${specialtyId}`,
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
        `${apiBase}/doctors/${doctorId}/available-dates`,
        { params }
      )
      return response
    } catch (error) {
      console.error('Error fetching doctor availability:', error)
      throw error
    }
  }

  interface PatientAppointment {
    id: string
    doctor_name: string
    specialty_name: string
    date: string      // YYYY-MM-DD
    time: string      // HH:MM
    hospital: string | null
    status: string
  }

  const fetchPatientAppointments = async (patientId: string) => {
    try {
      const response = await $fetch<PatientAppointment[]>(
        `${apiBase}/patients/${patientId}/appointments`
      )
      return response
    } catch (error) {
      console.error('Error fetching patient appointments:', error)
      throw error
    }
  }

  interface BookAppointmentRequest {
    patient_id: string
    date: string  // YYYY-MM-DD format
    time?: string // HH:MM format (optional for now)
  }

  const bookAppointment = async (doctorId: string, request: BookAppointmentRequest) => {
    try {
      console.log('Sending POST request:', {
        url: `${apiBase}/doctors/${doctorId}/appointments`,
        body: request,
        bodyJSON: JSON.stringify(request)
      })
      const response = await $fetch(
        `${apiBase}/doctors/${doctorId}/appointments`,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(request)
        }
      )
      return response
    } catch (error) {
      console.error('Error booking appointment:', error)
      throw error
    }
  }

  return {
    fetchSpecialties,
    fetchDoctorsBySpecialty,
    fetchDoctorAvailability,
    fetchPatientAppointments,
    bookAppointment
  }
}
