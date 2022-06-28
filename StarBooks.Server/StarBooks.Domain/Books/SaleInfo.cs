using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class SaleInfo
{
    [JsonPropertyName("country")]
    public string? Country { get; set; }

    [JsonPropertyName("isEbook")]
    public bool? IsEbook { get; set; }

    [JsonPropertyName("listPrice")]
    public Price ListPrice { get; set; }

    [JsonPropertyName("retailPrice")]
    public Price RetailPrice { get; set; }
}
