import { useAvatar } from "~/composables/useAvatar";
import type { ComplaintRow } from "~/queries/complaints";
import { formatPersonName } from "~/utils/personName";

export function complaintCustomerLabel(row: ComplaintRow | null): string {
  const customer = row?.customer;

  if (!customer) return "Unknown customer";

  const name = formatPersonName(customer.firstName, customer.lastName);

  return name || customer.email;
}

export function complaintCustomerAvatar(row: ComplaintRow | null) {
  const { initials } = useAvatar();

  return {
    src: row?.customer?.picture || undefined,
    alt: complaintCustomerLabel(row),
    text: initials(row?.customer?.firstName, row?.customer?.lastName),
  };
}
