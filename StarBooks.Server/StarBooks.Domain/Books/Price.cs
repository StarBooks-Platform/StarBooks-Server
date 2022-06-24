using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class Price
{
    [JsonPropertyName("amount")]
    decimal Amount { get; set; }

    [JsonPropertyName("currencyCode")]
    string CurrencyCode { get; set; }
}
