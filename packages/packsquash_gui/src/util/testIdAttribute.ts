export default (id: string) =>
  import.meta.env.MODE !== "production" ? { "data-test-id": id } : {};
