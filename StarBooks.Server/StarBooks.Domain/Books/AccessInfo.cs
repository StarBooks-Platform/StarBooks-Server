using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Owned]
public class AccessInfo
{
    [JsonPropertyName("country")]
    public string? Country { get; set; }

    [JsonPropertyName("viewability")]
    public string? Viewability { get; set; }

    [JsonPropertyName("embeddable")]
    public bool? Embeddable { get; set; }

    [JsonPropertyName("publicDomain")]
    public bool? PublicDomain { get; set; }

    [JsonPropertyName("textToSpeechPermission")]
    public string? TextToSpeechPermission { get; set; }

    [JsonPropertyName("epub")]
    public EResource Epub { get; set; }

    [JsonPropertyName("pdf")]
    public EResource Pdf { get; set; }

    [JsonPropertyName("webReaderLink")]
    public string? WebReaderLink { get; set; }

    [JsonPropertyName("quoteSharingAllowed")]
    public bool? QuoteSharingAllowed { get; set; }
}
