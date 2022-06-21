using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Table("prices"), Owned]
public record Price(
    [property: JsonIgnore, Key] Guid Id,
    [property: JsonPropertyName("amount")] decimal Amount,
    [property: JsonPropertyName("currencyCode")] string CurrencyCode
);
