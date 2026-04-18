import {json} from "@sveltejs/kit"
import type {RequestHandler} from "./$types"

export const GET: RequestHandler = async ({url}) => {
  const query = url.searchParams.get("q")?.toLowerCase() || ""

  // Имитация базы данных/результатов от Rust
  const allResults = [
    {
      id: "1",
      type: "document",
      title: "Годовой отчет 2023.pdf",
      description: "Полная финансовая аналитика за прошлый год",
      metadata: {size: "15.4 MB", author: "Finance Dept"},
    },
    {
      id: "2",
      type: "person",
      title: "Александр Пушкин",
      description: "Senior Rust Developer",
      metadata: {
        department: "Core Infrastructure",
        avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Alex",
      },
    },
    {
      id: "3",
      type: "document",
      title: "Инструкция по безопасности.docx",
      description: "Важные правила работы в офисе",
      metadata: {size: "1.2 MB"},
    },
    {
      id: "4",
      type: "person",
      title: "Мария Кюри",
      description: "Lead Research Scientist",
      metadata: {
        department: "R&D",
        avatar: "https://api.dicebear.com/7.x/avataaars/svg?seed=Maria",
      },
    },
    {
      id: "5",
      type: "app",
      title: "Jira Dashboard",
      description: "Система управления задачами проекта",
      metadata: {url: "https://jira.company.com"},
    },
  ]

  // Простая фильтрация для теста
  const filtered = allResults.filter(
    (item) => item.title.toLowerCase().includes(query) || item.description.toLowerCase().includes(query)
  )

  // Имитируем небольшую задержку от Rust-кластера
  await new Promise((resolve) => setTimeout(resolve, 400))

  return json(filtered)
}
